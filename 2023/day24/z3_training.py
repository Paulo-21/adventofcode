from z3 import  *

#cx , cy, cz, cvx, cvy, cvz = 19, 13, 30, -2,  1, -2
#cx2 , cy2, cz2, cvx2, cvy2, cvz2 = 18, 19, 22, -1, -1, -2

#f = open("example").read().splitlines()
f = open("input").read().splitlines()
x = Real('x')
y = Real('y')
z = Real('z')
vx = Real('vx')
vy = Real('vy')
vz = Real('vz')
s = Solver()
i = 0
for line in f:
    cos = line.split("@ ")
    cord = cos[0].split(", ")
    vel = cos[1].split(", ")
    t = Real('t'+str(i))
    s.add(x + vx*t == t*int(vel[0]) + int(cord[0]), y + vy*t == t*int(vel[1]) + int(cord[1]), z + vz*t == t*int(vel[2]) + int(cord[2]))
    i+=1

s.check()
print(s.model().eval(x+y+z))
