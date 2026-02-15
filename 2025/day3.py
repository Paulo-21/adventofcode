file = open("day3.input")
#file = open("exe.exemple")
res = 0
for line in file:
    max = 0
    next_m = 0
    l = list(line)
    #print(l[:-1])
    for k, n in enumerate(l[:-1]):
        n = int(n)
        if n > max and k < (len(l)-2):
            #print("m",n)
            max = n
            next_m = 0
        elif n > next_m:
            #print("mi",n)
            next_m = n
    print("------------", max*10+next_m)
    res += max*10+next_m
print(res)
