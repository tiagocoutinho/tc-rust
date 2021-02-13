def fib(num):
    if num < 2:
        return 1
    return fib(num - 2) + fib(num - 1)

print("fib(38) = {}".format(fib(38)))
