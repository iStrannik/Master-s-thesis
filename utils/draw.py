import matplotlib.pyplot as plt
import sys

x = []
y = []

with open(sys.argv[1], 'r') as file:
    for i in file.readlines():
        a, b = list(map(int, i.rstrip().split()))[:2]
        x.append(a)
        y.append(b)

plt.plot(x, y)
plt.savefig('columns_to_kbytes.png')
