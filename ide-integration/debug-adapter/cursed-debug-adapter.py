#!/usr/bin/env python3
"""
CURSED Debug Adapter Protocol Implementation
Provides advanced debugging capabilities for CURSED applications.
"""

import asyncio
import json
import sys
import subprocess
import threading
import time
import os
import signal
from pathlib import Path
from typing import Dict, List, Optional, Any
from dataclasses import dataclass
from enum import Enum

class StoppedReason(Enum):
    STEP = "step"
    BREAKPOINT = "breakpoint"
    EXCEPTION = "exception"
    PAUSE = "pause"
    ENTRY = "entry"

@dataclass
class Breakpoint:
    id: int
    line: int
    column: Optional[int] = None
    condition: Optional[str] = None
    hit_condition: Optional[str] = None
    verified: bool = False

@dataclass
class Variable:
    name: str
    value: str
    type: str
    variables_reference: int = 0
    named_variables: Optional[int] = None
    indexed_variables: Optional[int] = None

@dataclass
class StackFrame:
    id: int
    name: str
    source: Optional[str] = None
    line: int = 0
    column: int = 0
    end_line: Optional[int] = None
    end_column: Optional[int] = None

@dataclass
class Thread:
    id: int
    name: str

class CursedDebugAdapter:
    """CURSED Debug Adapter Protocol implementation."""
    
    def __init__(self):
        self.seq = 0
        self.client_connected = False
        self.cursed_process = None
        self.breakpoints: Dict[str, List[Breakpoint]] = {}
        self.next_breakpoint_id = 1
        self.threads: Dict[int, Thread] = {}
        self.stack_frames: Dict[int, List[StackFrame]] = {}
        self.variables: Dict[int, List[Variable]] = {}
        self.next_variable_ref = 1
        self.stopped_threads: set = set()
        self.configuration_done = False
        
    async def start(self):
        """Start the debug adapter."""
        try:
            while True:
                line = await self.read_message()
                if line:
                    await self.handle_message(line)
                else:
                    break
        except KeyboardInterrupt:
            await self.cleanup()
    
    async def read_message(self) -> Optional[str]:
        """Read a message from stdin."""
        try:
            line = sys.stdin.readline()
            if not line:
                return None
            return line.strip()
        except EOFError:
            return None
    
    async def send_message(self, message: Dict[str, Any]):
        """Send a message to the client."""
        message['seq'] = self.seq
        self.seq += 1
        
        json_str = json.dumps(message, separators=(',', ':'))
        content_length = len(json_str.encode('utf-8'))
        
        output = f"Content-Length: {content_length}\r\n\r\n{json_str}"
        sys.stdout.write(output)
        sys.stdout.flush()
    
    async def send_response(self, request: Dict[str, Any], success: bool = True, body: Optional[Dict] = None, message: Optional[str] = None):
        """Send a response to a request."""
        response = {
            'type': 'response',
            'request_seq': request['seq'],
            'command': request['command'],
            'success': success
        }
        
        if body:
            response['body'] = body
        if message:
            response['message'] = message
            
        await self.send_message(response)
    
    async def send_event(self, event: str, body: Optional[Dict] = None):
        """Send an event to the client."""
        message = {
            'type': 'event',
            'event': event
        }
        
        if body:
            message['body'] = body
            
        await self.send_message(message)
    
    async def handle_message(self, line: str):
        """Handle incoming message."""
        try:
            if line.startswith('Content-Length:'):
                # Read the actual JSON message
                content_length = int(line.split(':')[1].strip())
                sys.stdin.readline()  # Skip empty line
                json_data = sys.stdin.read(content_length)
                message = json.loads(json_data)
            else:
                message = json.loads(line)
            
            await self.dispatch_request(message)
            
        except json.JSONDecodeError as e:
            await self.send_event('output', {
                'category': 'stderr',
                'output': f"JSON decode error: {e}\n"
            })
        except Exception as e:
            await self.send_event('output', {
                'category': 'stderr',
                'output': f"Error handling message: {e}\n"
            })
    
    async def dispatch_request(self, request: Dict[str, Any]):
        """Dispatch request to appropriate handler."""
        command = request.get('command')
        
        handlers = {
            'initialize': self.handle_initialize,
            'launch': self.handle_launch,
            'attach': self.handle_attach,
            'configurationDone': self.handle_configuration_done,
            'setBreakpoints': self.handle_set_breakpoints,
            'continue': self.handle_continue,
            'next': self.handle_next,
            'stepIn': self.handle_step_in,
            'stepOut': self.handle_step_out,
            'pause': self.handle_pause,
            'stackTrace': self.handle_stack_trace,
            'scopes': self.handle_scopes,
            'variables': self.handle_variables,
            'evaluate': self.handle_evaluate,
            'threads': self.handle_threads,
            'disconnect': self.handle_disconnect,
            'restart': self.handle_restart,
        }
        
        handler = handlers.get(command)
        if handler:
            await handler(request)
        else:
            await self.send_response(request, success=False, message=f"Unknown command: {command}")
    
    async def handle_initialize(self, request: Dict[str, Any]):
        """Handle initialize request."""
        capabilities = {
            'supportsConfigurationDoneRequest': True,
            'supportsFunctionBreakpoints': False,
            'supportsConditionalBreakpoints': True,
            'supportsHitConditionalBreakpoints': True,
            'supportsEvaluateForHovers': True,
            'exceptionBreakpointFilters': [
                {
                    'filter': 'uncaught',
                    'label': 'Uncaught Exceptions',
                    'default': True
                },
                {
                    'filter': 'all',
                    'label': 'All Exceptions',
                    'default': False
                }
            ],
            'supportsStepBack': False,
            'supportsSetVariable': True,
            'supportsRestartFrame': False,
            'supportsGotoTargetsRequest': False,
            'supportsStepInTargetsRequest': False,
            'supportsCompletionsRequest': True,
            'completionTriggerCharacters': ['.', '['],
            'supportsModulesRequest': False,
            'additionalModuleColumns': [],
            'supportedChecksumAlgorithms': [],
            'supportsRestartRequest': True,
            'supportsExceptionOptions': True,
            'supportsValueFormattingOptions': True,
            'supportsExceptionInfoRequest': True,
            'supportTerminateDebuggee': True,
            'supportSuspendDebuggee': True,
            'supportsDelayedStackTraceLoading': True,
            'supportsLoadedSourcesRequest': False,
            'supportsLogPoints': True,
            'supportsTerminateThreadsRequest': False,
            'supportsSetExpression': False,
            'supportsTerminateRequest': True,
            'supportsDataBreakpoints': False,
            'supportsReadMemoryRequest': True,
            'supportsWriteMemoryRequest': False,
            'supportsDisassembleRequest': True
        }
        
        await self.send_response(request, body=capabilities)
        await self.send_event('initialized')
    
    async def handle_launch(self, request: Dict[str, Any]):
        """Handle launch request."""
        args = request.get('arguments', {})
        program = args.get('program')
        
        if not program:
            await self.send_response(request, success=False, message="No program specified")
            return
        
        program_path = Path(program)
        if not program_path.exists():
            await self.send_response(request, success=False, message=f"Program not found: {program}")
            return
        
        # Launch CURSED program with debug support
        cmd = [
            'cursed-zig',
            '--debug',
            '--debug-port', '0',  # Use random port
            program
        ]
        
        cmd_args = args.get('args', [])
        if cmd_args:
            cmd.extend(cmd_args)
        
        cwd = args.get('cwd', str(program_path.parent))
        env = dict(os.environ)
        env.update(args.get('env', {}))
        
        try:
            self.cursed_process = subprocess.Popen(
                cmd,
                cwd=cwd,
                env=env,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                stdin=subprocess.PIPE,
                universal_newlines=True
            )
            
            # Start monitoring process
            threading.Thread(target=self.monitor_process, daemon=True).start()
            
            await self.send_response(request)
            
            # Simulate initial thread
            main_thread = Thread(id=1, name="main")
            self.threads[1] = main_thread
            
            if args.get('stopOnEntry', False):
                await self.send_event('stopped', {
                    'reason': StoppedReason.ENTRY.value,
                    'threadId': 1,
                    'allThreadsStopped': True
                })
            
        except Exception as e:
            await self.send_response(request, success=False, message=f"Failed to launch program: {e}")
    
    async def handle_attach(self, request: Dict[str, Any]):
        """Handle attach request."""
        args = request.get('arguments', {})
        process_id = args.get('processId')
        
        if not process_id:
            await self.send_response(request, success=False, message="No process ID specified")
            return
        
        # Attach to running CURSED process
        try:
            # This would require CURSED runtime support for attaching debugger
            await self.send_response(request, success=False, message="Attach not yet implemented")
        except Exception as e:
            await self.send_response(request, success=False, message=f"Failed to attach: {e}")
    
    async def handle_configuration_done(self, request: Dict[str, Any]):
        """Handle configuration done request."""
        self.configuration_done = True
        await self.send_response(request)
    
    async def handle_set_breakpoints(self, request: Dict[str, Any]):
        """Handle set breakpoints request."""
        args = request.get('arguments', {})
        source = args.get('source', {})
        file_path = source.get('path', '')
        
        breakpoints = []
        requested_breakpoints = args.get('breakpoints', [])
        
        # Clear existing breakpoints for this file
        self.breakpoints[file_path] = []
        
        for bp_data in requested_breakpoints:
            bp = Breakpoint(
                id=self.next_breakpoint_id,
                line=bp_data['line'],
                column=bp_data.get('column'),
                condition=bp_data.get('condition'),
                hit_condition=bp_data.get('hitCondition'),
                verified=True  # Assume all breakpoints are valid for now
            )
            self.next_breakpoint_id += 1
            
            self.breakpoints[file_path].append(bp)
            
            breakpoints.append({
                'id': bp.id,
                'verified': bp.verified,
                'line': bp.line,
                'column': bp.column,
                'message': 'Breakpoint set' if bp.verified else 'Invalid breakpoint location'
            })
        
        await self.send_response(request, body={'breakpoints': breakpoints})
    
    async def handle_continue(self, request: Dict[str, Any]):
        """Handle continue request."""
        args = request.get('arguments', {})
        thread_id = args.get('threadId')
        
        if thread_id in self.stopped_threads:
            self.stopped_threads.remove(thread_id)
        
        # Send continue command to CURSED process
        if self.cursed_process:
            self.cursed_process.stdin.write("continue\n")
            self.cursed_process.stdin.flush()
        
        await self.send_response(request, body={'allThreadsContinued': False})
    
    async def handle_next(self, request: Dict[str, Any]):
        """Handle next (step over) request."""
        args = request.get('arguments', {})
        thread_id = args.get('threadId')
        
        # Send step over command to CURSED process
        if self.cursed_process:
            self.cursed_process.stdin.write("next\n")
            self.cursed_process.stdin.flush()
        
        await self.send_response(request)
        
        # Simulate step completion
        await asyncio.sleep(0.1)
        await self.send_event('stopped', {
            'reason': StoppedReason.STEP.value,
            'threadId': thread_id,
            'allThreadsStopped': False
        })
    
    async def handle_step_in(self, request: Dict[str, Any]):
        """Handle step in request."""
        args = request.get('arguments', {})
        thread_id = args.get('threadId')
        
        # Send step into command to CURSED process
        if self.cursed_process:
            self.cursed_process.stdin.write("step\n")
            self.cursed_process.stdin.flush()
        
        await self.send_response(request)
        
        # Simulate step completion
        await asyncio.sleep(0.1)
        await self.send_event('stopped', {
            'reason': StoppedReason.STEP.value,
            'threadId': thread_id,
            'allThreadsStopped': False
        })
    
    async def handle_step_out(self, request: Dict[str, Any]):
        """Handle step out request."""
        args = request.get('arguments', {})
        thread_id = args.get('threadId')
        
        # Send step out command to CURSED process
        if self.cursed_process:
            self.cursed_process.stdin.write("finish\n")
            self.cursed_process.stdin.flush()
        
        await self.send_response(request)
        
        # Simulate step completion
        await asyncio.sleep(0.1)
        await self.send_event('stopped', {
            'reason': StoppedReason.STEP.value,
            'threadId': thread_id,
            'allThreadsStopped': False
        })
    
    async def handle_pause(self, request: Dict[str, Any]):
        """Handle pause request."""
        args = request.get('arguments', {})
        thread_id = args.get('threadId')
        
        # Send pause command to CURSED process
        if self.cursed_process:
            self.cursed_process.send_signal(signal.SIGINT)
        
        await self.send_response(request)
        await self.send_event('stopped', {
            'reason': StoppedReason.PAUSE.value,
            'threadId': thread_id,
            'allThreadsStopped': True
        })
    
    async def handle_stack_trace(self, request: Dict[str, Any]):
        """Handle stack trace request."""
        args = request.get('arguments', {})
        thread_id = args.get('threadId')
        start_frame = args.get('startFrame', 0)
        levels = args.get('levels', 20)
        
        # Generate mock stack frames
        stack_frames = [
            {
                'id': 1000,
                'name': 'main',
                'source': {
                    'name': 'main.💀',
                    'path': '/path/to/main.💀'
                },
                'line': 45,
                'column': 12,
                'endLine': 45,
                'endColumn': 20
            },
            {
                'id': 1001,
                'name': 'handle_request',
                'source': {
                    'name': 'server.💀',
                    'path': '/path/to/server.💀'
                },
                'line': 123,
                'column': 8,
                'endLine': 123,
                'endColumn': 25
            }
        ]
        
        # Apply pagination
        paginated_frames = stack_frames[start_frame:start_frame + levels]
        
        await self.send_response(request, body={
            'stackFrames': paginated_frames,
            'totalFrames': len(stack_frames)
        })
    
    async def handle_scopes(self, request: Dict[str, Any]):
        """Handle scopes request."""
        args = request.get('arguments', {})
        frame_id = args.get('frameId')
        
        scopes = [
            {
                'name': 'Locals',
                'variablesReference': 2000,
                'expensive': False
            },
            {
                'name': 'Globals',
                'variablesReference': 2001,
                'expensive': False
            },
            {
                'name': 'Goroutines',
                'variablesReference': 2002,
                'expensive': True
            }
        ]
        
        await self.send_response(request, body={'scopes': scopes})
    
    async def handle_variables(self, request: Dict[str, Any]):
        """Handle variables request."""
        args = request.get('arguments', {})
        variables_reference = args.get('variablesReference')
        
        # Generate mock variables based on reference
        if variables_reference == 2000:  # Locals
            variables = [
                {
                    'name': 'request',
                    'value': 'HttpRequest { method: "GET", path: "/api/users" }',
                    'type': 'HttpRequest',
                    'variablesReference': 3000
                },
                {
                    'name': 'user_id',
                    'value': '123',
                    'type': 'drip',
                    'variablesReference': 0
                },
                {
                    'name': 'is_admin',
                    'value': 'based',
                    'type': 'lit',
                    'variablesReference': 0
                }
            ]
        elif variables_reference == 2001:  # Globals
            variables = [
                {
                    'name': 'CONFIG',
                    'value': 'Config { port: 8080, host: "localhost" }',
                    'type': 'Config',
                    'variablesReference': 3001
                },
                {
                    'name': 'DB_POOL',
                    'value': 'DatabasePool { active: 5, idle: 3 }',
                    'type': 'DatabasePool',
                    'variablesReference': 3002
                }
            ]
        elif variables_reference == 2002:  # Goroutines
            variables = [
                {
                    'name': 'goroutine_1',
                    'value': 'running',
                    'type': 'Goroutine',
                    'variablesReference': 0
                },
                {
                    'name': 'goroutine_2',
                    'value': 'waiting',
                    'type': 'Goroutine',
                    'variablesReference': 0
                }
            ]
        else:
            variables = []
        
        await self.send_response(request, body={'variables': variables})
    
    async def handle_evaluate(self, request: Dict[str, Any]):
        """Handle evaluate request."""
        args = request.get('arguments', {})
        expression = args.get('expression', '')
        context = args.get('context', 'watch')
        
        # Mock evaluation
        if expression == 'user_id':
            result = '123'
            type_name = 'drip'
        elif expression == 'request.method':
            result = '"GET"'
            type_name = 'tea'
        else:
            result = f'<evaluation of "{expression}" not implemented>'
            type_name = 'unknown'
        
        await self.send_response(request, body={
            'result': result,
            'type': type_name,
            'variablesReference': 0
        })
    
    async def handle_threads(self, request: Dict[str, Any]):
        """Handle threads request."""
        threads = [
            {'id': thread.id, 'name': thread.name}
            for thread in self.threads.values()
        ]
        
        await self.send_response(request, body={'threads': threads})
    
    async def handle_disconnect(self, request: Dict[str, Any]):
        """Handle disconnect request."""
        await self.cleanup()
        await self.send_response(request)
    
    async def handle_restart(self, request: Dict[str, Any]):
        """Handle restart request."""
        await self.cleanup()
        # Restart would re-launch the process
        await self.send_response(request)
    
    def monitor_process(self):
        """Monitor the CURSED process for output and events."""
        if not self.cursed_process:
            return
        
        try:
            while self.cursed_process.poll() is None:
                # Read stdout
                line = self.cursed_process.stdout.readline()
                if line:
                    asyncio.create_task(self.send_event('output', {
                        'category': 'stdout',
                        'output': line
                    }))
                
                # Read stderr
                error_line = self.cursed_process.stderr.readline()
                if error_line:
                    asyncio.create_task(self.send_event('output', {
                        'category': 'stderr',
                        'output': error_line
                    }))
                
                time.sleep(0.01)
            
            # Process exited
            exit_code = self.cursed_process.returncode
            asyncio.create_task(self.send_event('exited', {'exitCode': exit_code}))
            asyncio.create_task(self.send_event('terminated'))
            
        except Exception as e:
            asyncio.create_task(self.send_event('output', {
                'category': 'stderr',
                'output': f"Process monitoring error: {e}\n"
            }))
    
    async def cleanup(self):
        """Clean up resources."""
        if self.cursed_process:
            try:
                self.cursed_process.terminate()
                self.cursed_process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.cursed_process.kill()
            except Exception:
                pass
            finally:
                self.cursed_process = None

async def main():
    """Main entry point."""
    adapter = CursedDebugAdapter()
    await adapter.start()

if __name__ == '__main__':
    asyncio.run(main())
