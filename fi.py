#!/usr/bin/env python3


import numpy as np
from math import asinh, atan, sqrt, pi

n = (100, 25, 1)
dx = (5e-9, 5e-9, 3e-9)
mu0 = 4e-7 * pi
gamma = 2.211e5
ms = 8e5
A = 1.3e-11
alpha = 0.02

eps = 1e-18


def f(p):
    x, y, z = abs(p[0]), abs(p[1]), abs(p[2])
    return y/2.0 * (z**2 - x**2) * asinh(y/(sqrt(x**2 + z**2) + eps))
    + z/2.0 * (y**2 - x**2) * asinh(z/(sqrt(x**2 + y**2) + eps))
    - x*y*z*atan(y*z/(x * sqrt(x**2 + y**2 + z**2) + eps))
    + 1.0/6.0*(2*x**2 - y**2 - z**2)*sqrt(x**2 + y**2 + z**2)


def g(p):
    x, y, z = p[0], p[1], abs(p[2])
    return x*y*z*asinh(z/(sqrt(x**2+y**2)+eps))
    + y/6.0*(3.0*z**2-y**2)*asinh(x/(sqrt(y**2+z**2)+eps))
    + x/6.0*(3.0*z**2-x**2)*asinh(y/(sqrt(x**2+z**2)+eps))
    - z**3/6.0 * atan(x*y/(z*sqrt(x**2+y**2+z**2)+eps))
    - z*y**2/2.0*atan(x*z/(y*sqrt(x**2+y**2+z**2)+eps))
    - z*x**2/2.0*atan(y*z/(x*sqrt(x**2+y**2+z**2)+eps))
    - x*y*sqrt(x**2+y**2+z**2)/3.0


# print(g([0., 0., 0.]))
# print(g([1., 1., 1.]))
# print("0.1 ->")
# print(g([0.1, 0.1, 0.1]))
# print("-0.1 ->")
# print(g([-0.1, -0.1, -0.1]))

# print(f([0., 0., 0.]))
# print(f([-0.1, -0.1, -0.1]))
# print(f([0.1, 0.1, 0.1]))
# print(f([-1., -1., -1.]))
# print(f([1., 1., 1.]))

print(g([0., 0., 0.]))
print(g([-0.1, -0.1, -0.1]))
print(g([0.1, 0.1, 0.1]))
print(g([-1., -1., -1.]))
print(g([1., 1., 1.]))
