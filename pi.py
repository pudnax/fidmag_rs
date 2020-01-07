from math import asinh, atan, sqrt, pi
import numpy as np

n = (5, 3, 1)


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
    return + y/2.0 * (z**2 - x**2) * asinh(y/(sqrt(x**2 + z**2) + eps))
    + z/2.0 * (y**2 - x**2) * asinh(z/(sqrt(x**2 + y**2) + eps))
    - x*y*z*atan(y*z/(x * sqrt(x**2 + y**2 + z**2) + eps))
    + 1.0/6.0*(2*x**2 - y**2 - z**2)*sqrt(x**2 + y**2 + z**2)


def g(p):
    x, y, z = p[0], p[1], abs(p[2])
    return + x*y*z*asinh(z/(sqrt(x**2+y**2)+eps))
    + y/6.0*(3.0*z**2-y**2)*asinh(x/(sqrt(y**2+z**2)+eps))
    + x/6.0*(3.0*z**2-x**2)*asinh(y/(sqrt(x**2+z**2)+eps))
    - z**3/6.0 * atan(x*y/(z*sqrt(x**2+y**2+z**2)+eps))
    - z*y**2/2.0*atan(x*z/(y*sqrt(x**2+y**2+z**2)+eps))
    - z*x**2/2.0*atan(y*z/(x*sqrt(x**2+y**2+z**2)+eps))
    - x*y*sqrt(x**2+y**2+z**2)/3.0


def set_n_demag(c, permute, func):
    it = np.nditer(m_pad[:, :, :, c], flags=[
                   'multi_index'], op_flags=['writeonly'])
    while not it.finished:
        value = 0.0
        for i in np.rollaxis(np.indices((2,)*6), 0, 7).reshape(64, 6):
            idx = map(lambda k: (
                it.multi_index[k]+n[k]-1) % (2*n[k]-1)-n[k]+1, range(3))
            value += (-1) ** sum(i) * \
                func(map(lambda j: (idx[j]+i[j]-i[j+3])*dx[j], permute))
        it[0] = -value/(4*pi*np.prod(dx))
        it.iternext()


np.set_printoptions(threshold=np.inf)

m = 5
n = [2*i-1 for i in n] + [m]
m_pad = np.linspace(1, 1, n[0]*n[1]*n[2]*n[3]).reshape(n)
for i in range(m):
    m_pad[:, :, :m] = [x if x % 2 == 0 else x-1 for x in range(m)]
# m_pad = np.zeros([2*i-1 for i in n] + [6])

# for i, t in enumerate(((f, 0, 1, 2), (g, 0, 1, 2), (g, 0, 2, 1),
#                        (f, 1, 2, 0), (g, 1, 2, 0), (f, 2, 0, 1))):
#     set_n_demag(i, t[1:], t[0])
m_pad = np.fft.fftn(m_pad, axes=filter(lambda i: n[i] > 1, range(3)))
print(m_pad)
