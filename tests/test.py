import tsl

t = tsl.TSL(100)
t.append(1, b"hello")
t.append(2, b"world")

print(t.latest(2))
print(t.range_query(1, 2))