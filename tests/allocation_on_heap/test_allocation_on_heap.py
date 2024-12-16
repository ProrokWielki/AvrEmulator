import subprocess
import threading
import os

def test_allocation_on_heap():
    proc = subprocess.Popen(["cargo", "run", os.path.dirname(__file__)+"/build/allocation_on_heap.hex", "-vv"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    timer = threading.Timer(5, proc.kill)
    
    try:
        timer.start()
        _, stderr = proc.communicate()
    finally:
        timer.cancel()
        
    output = stderr.decode("utf-8").split("\n")

    assert len(output) > 100
    assert sum('rjmp -1' in instruction for instruction in output) > 100