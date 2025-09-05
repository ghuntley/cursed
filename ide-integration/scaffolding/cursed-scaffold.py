#!/usr/bin/env python3
"""
CURSED Project Scaffolding Tool
Creates new CURSED projects from templates with intelligent configuration.
"""

import os
import sys
import shutil
import argparse
import json
import re
from pathlib import Path
from typing import Dict, List, Optional
from datetime import datetime

class CursedScaffold:
    """CURSED project scaffolding and template system."""
    
    def __init__(self):
        self.script_dir = Path(__file__).parent
        self.templates_dir = self.script_dir / "templates"
        self.default_config = {
            "author_name": os.getenv("USER", "Developer"),
            "author_email": f"{os.getenv('USER', 'developer')}@example.com",
            "github_username": os.getenv("USER", "developer"),
            "license": "MIT OR Apache-2.0",
            "cursed_version": "1.0.0"
        }
    
    def get_available_templates(self) -> List[str]:
        """Get list of available project templates."""
        if not self.templates_dir.exists():
            return []
        
        templates = []
        for item in self.templates_dir.iterdir():
            if item.is_dir() and (item / "template.json").exists():
                templates.append(item.name)
        
        return sorted(templates)
    
    def load_template_config(self, template_name: str) -> Dict:
        """Load template configuration."""
        template_dir = self.templates_dir / template_name
        config_file = template_dir / "template.json"
        
        if not config_file.exists():
            raise ValueError(f"Template configuration not found: {config_file}")
        
        with open(config_file, 'r') as f:
            return json.load(f)
    
    def get_user_input(self, template_config: Dict) -> Dict[str, str]:
        """Get user input for template variables."""
        variables = {}
        
        # Start with default configuration
        variables.update(self.default_config)
        
        # Get project-specific information
        print(f"\n🚀 Creating new {template_config['name']} project")
        print(f"📝 {template_config['description']}\n")
        
        # Required variables
        required_vars = template_config.get('required_variables', [])
        for var in required_vars:
            var_config = template_config['variables'][var]
            
            prompt = var_config.get('prompt', f"Enter {var}")
            default = var_config.get('default', variables.get(var, ''))
            
            if default:
                value = input(f"{prompt} [{default}]: ").strip()
                if not value:
                    value = default
            else:
                while True:
                    value = input(f"{prompt}: ").strip()
                    if value:
                        break
                    print("This field is required.")
            
            variables[var] = value
        
        # Optional variables
        optional_vars = template_config.get('optional_variables', [])
        for var in optional_vars:
            var_config = template_config['variables'][var]
            
            prompt = var_config.get('prompt', f"Enter {var}")
            default = var_config.get('default', variables.get(var, ''))
            
            value = input(f"{prompt} [{default}]: ").strip()
            if not value:
                value = default
            
            variables[var] = value
        
        # Auto-generated variables
        variables['created_date'] = datetime.now().isoformat()
        variables['year'] = str(datetime.now().year)
        
        return variables
    
    def process_template_file(self, file_path: Path, variables: Dict[str, str]) -> str:
        """Process a template file, replacing variables."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except UnicodeDecodeError:
            # Handle binary files
            return None
        
        # Replace template variables
        for var, value in variables.items():
            # Handle different variable formats
            patterns = [
                f"{{{{{{var.upper()}}}}}",  # {{VAR}}
                f"{{{{{{var.lower()}}}}}",  # {{var}}
                f"{{{{{{var}}}}}",          # {{var}} (as-is)
            ]
            
            for pattern in patterns:
                content = content.replace(pattern, str(value))
        
        return content
    
    def copy_template_files(self, template_dir: Path, target_dir: Path, variables: Dict[str, str]):
        """Copy and process template files to target directory."""
        template_config_file = template_dir / "template.json"
        
        for item in template_dir.rglob("*"):
            if item == template_config_file or item.name.startswith('.'):
                continue
            
            relative_path = item.relative_to(template_dir)
            target_path = target_dir / relative_path
            
            # Process path template variables
            target_path_str = str(target_path)
            for var, value in variables.items():
                target_path_str = target_path_str.replace(f"{{{{{var}}}}}", value)
            target_path = Path(target_path_str)
            
            if item.is_dir():
                target_path.mkdir(parents=True, exist_ok=True)
            else:
                target_path.parent.mkdir(parents=True, exist_ok=True)
                
                # Process file content if it's a text file
                processed_content = self.process_template_file(item, variables)
                if processed_content is not None:
                    with open(target_path, 'w', encoding='utf-8') as f:
                        f.write(processed_content)
                else:
                    # Copy binary file as-is
                    shutil.copy2(item, target_path)
    
    def run_post_creation_hooks(self, target_dir: Path, template_config: Dict, variables: Dict[str, str]):
        """Run post-creation hooks if specified in template."""
        hooks = template_config.get('post_creation_hooks', [])
        
        if not hooks:
            return
        
        print("\n🔧 Running post-creation setup...")
        
        for hook in hooks:
            print(f"   Running: {hook}")
            
            # Replace variables in hook command
            processed_hook = hook
            for var, value in variables.items():
                processed_hook = processed_hook.replace(f"{{{{{var}}}}}", value)
            
            # Execute hook in target directory
            result = os.system(f"cd {target_dir} && {processed_hook}")
            if result != 0:
                print(f"   ⚠️  Warning: Hook failed with exit code {result}")
    
    def create_project(self, template_name: str, project_name: str, target_dir: Optional[Path] = None):
        """Create a new project from template."""
        if template_name not in self.get_available_templates():
            raise ValueError(f"Template '{template_name}' not found")
        
        template_dir = self.templates_dir / template_name
        template_config = self.load_template_config(template_name)
        
        # Determine target directory
        if target_dir is None:
            target_dir = Path.cwd() / project_name
        
        if target_dir.exists():
            response = input(f"Directory '{target_dir}' already exists. Overwrite? [y/N]: ")
            if response.lower() != 'y':
                print("Aborted.")
                return
            shutil.rmtree(target_dir)
        
        # Get template variables
        variables = self.get_user_input(template_config)
        variables['project_name'] = project_name
        variables['project_name_snake'] = re.sub(r'[^a-zA-Z0-9]', '_', project_name).lower()
        variables['project_name_kebab'] = re.sub(r'[^a-zA-Z0-9]', '-', project_name).lower()
        
        # Create target directory
        target_dir.mkdir(parents=True, exist_ok=True)
        
        print(f"\n📁 Creating project in: {target_dir}")
        
        # Copy and process template files
        self.copy_template_files(template_dir, target_dir, variables)
        
        # Run post-creation hooks
        self.run_post_creation_hooks(target_dir, template_config, variables)
        
        # Success message
        print(f"\n✅ Project '{project_name}' created successfully!")
        print(f"📂 Location: {target_dir}")
        
        # Show next steps
        next_steps = template_config.get('next_steps', [])
        if next_steps:
            print("\n📋 Next steps:")
            for i, step in enumerate(next_steps, 1):
                # Process variables in steps
                processed_step = step
                for var, value in variables.items():
                    processed_step = processed_step.replace(f"{{{{{var}}}}}", value)
                print(f"   {i}. {processed_step}")
        
        print(f"\n🚀 To get started:")
        print(f"   cd {target_dir}")
        print(f"   cursed-zig src/main.💀")

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="CURSED Project Scaffolding Tool",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s list                           # List available templates
  %(prog)s new web-app my-api            # Create web app project
  %(prog)s new cli-tool my-cli           # Create CLI tool project
  %(prog)s new library math-utils        # Create library project
        """
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # List command
    list_parser = subparsers.add_parser('list', help='List available templates')
    
    # New command
    new_parser = subparsers.add_parser('new', help='Create new project from template')
    new_parser.add_argument('template', help='Template name')
    new_parser.add_argument('name', help='Project name')
    new_parser.add_argument('--dir', type=Path, help='Target directory (default: current dir)')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    scaffold = CursedScaffold()
    
    try:
        if args.command == 'list':
            templates = scaffold.get_available_templates()
            if not templates:
                print("No templates found.")
                return
            
            print("Available CURSED project templates:")
            for template in templates:
                try:
                    config = scaffold.load_template_config(template)
                    print(f"  {template:<20} - {config.get('description', 'No description')}")
                except Exception as e:
                    print(f"  {template:<20} - (invalid template: {e})")
        
        elif args.command == 'new':
            scaffold.create_project(args.template, args.name, args.dir)
        
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()
