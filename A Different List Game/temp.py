import primefac
import sys

n = int( sys.argv[1] )
factors = list( primefac.primefac(n) )
print '\n'.join(map(str, factors))