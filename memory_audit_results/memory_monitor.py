import subprocess
import psutil
import time
import os

def monitor_process(command, iterations=50):
    memory_usage = []
    
    for i in range(iterations):
        # Start process
        proc = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        
        try:
            # Monitor memory during execution
            process = psutil.Process(proc.pid)
            max_memory = 0
            
            while proc.poll() is None:
                try:
                    mem_info = process.memory_info()
                    current_memory = mem_info.rss / 1024 / 1024  # MB
                    max_memory = max(max_memory, current_memory)
                    time.sleep(0.001)  # 1ms sampling
                except psutil.NoSuchProcess:
                    break
            
            proc.wait()
            memory_usage.append(max_memory)
            
            if i % 10 == 0:
                print(f"Iteration {i}: {max_memory:.2f} MB")
                
        except Exception as e:
            print(f"Error monitoring iteration {i}: {e}")
            proc.terminate()
    
    return memory_usage

if __name__ == "__main__":
    print("Monitoring cursed-zig memory usage...")
    cursed_zig_memory = monitor_process(["../zig-out/bin/cursed-zig", "stress_test.💀"])
    
    print("\nMonitoring cursed-stable memory usage...")
    cursed_stable_memory = monitor_process(["../zig-out/bin/cursed-stable", "stress_test.💀"])
    
    print(f"\nResults:")
    print(f"cursed-zig: avg={sum(cursed_zig_memory)/len(cursed_zig_memory):.2f} MB, max={max(cursed_zig_memory):.2f} MB")
    print(f"cursed-stable: avg={sum(cursed_stable_memory)/len(cursed_stable_memory):.2f} MB, max={max(cursed_stable_memory):.2f} MB")
