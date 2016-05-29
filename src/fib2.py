from threading import Lock, Thread

def fib(num):
    if num < 2:
        return 1
    return fib(num - 2) + fib(num - 1)

def thread_prog(mutex, results, i):
    rv = fib(i)
    with mutex:
        results[i] = rv

def main():
    mutex = Lock()
    results = {}

    threads = []
    for i in xrange(35):
        thread = Thread(target=thread_prog, args=(mutex, results, i))
        threads.append(thread)
        thread.start()

    for thread in threads:
        thread.join()

    for i, rv in sorted(results.items()):
        print "fib({}) = {}".format(i, rv)

main()
