import pyo3_example
import time
import threading

def display(myClass):
    myClass.display()

if __name__ == '__main__':
    myClass = pyo3_example.MyClass("Test", "This is a test of my pyo3 example")
    print("Create the thread")
    thread = threading.Thread(target=display, args=(myClass,), daemon=True)
    print("Start the thread")
    thread.start()

    while True:
        print("Waiting")
        time.sleep(2)