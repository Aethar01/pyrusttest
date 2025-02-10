from pyrusttestrs import hello_from_rust, printrs


def main():
    print("Hello World from python!")
    print(hello_from_rust())
    printrs("Hello from rust!")
