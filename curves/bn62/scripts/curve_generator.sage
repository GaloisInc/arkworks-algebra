modulus = 3866540040962951063

assert(modulus.is_prime())

Fp = GF(modulus)

def f(x):
	return x^3 + 29

# X generator
generator = Fp(0);
for x in range(0, 20):
    i = Fp(x);
    neg_i = Fp(-i)
    y = f(i)
    neg_y = f(neg_i)
    print("x: %d" % i)
    print("y2: %d" % y)
    print("-x: %d" % neg_i)
    print("-y2: %d" % neg_y)
    if y.is_square():
        generator = i
        # print("Generator: %d" % generator)
        break
    elif neg_y.is_square():
        generator = neg_i
        # print("Generator: %d" % generator)
        break

y = f(generator).sqrt()
print("Generator: (%d, %d)" % (generator, y))
print("Equal (y^2 = x^3 + 29)?: (%d, %d)" % (y^2, f(generator)))

