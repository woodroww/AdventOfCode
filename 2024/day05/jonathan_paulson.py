import sys
from collections import defaultdict, Counter, deque

sys.setrecursionlimit(10**6)
infile = sys.argv[1] if len(sys.argv)>=2 else 'input.txt'
p1 = 0
p2 = 0
D = open(infile).read().strip()

# E[x] is the set of pages that must come before x
# pages_before_key 
E = defaultdict(set)
# ER[x] is the set of pages that must come after x
# pages_after_key 
ER = defaultdict(set)
edges, queries = D.split('\n\n')
for line in edges.split('\n'):
    x,y = line.split('|')
    x,y = int(x), int(y)
    E[y].add(x)
    ER[x].add(y)

for query in queries.split('\n'):
    update = [int(x) for x in query.split(',')]
    assert len(update)%2==1
    ok = True
    for i,x in enumerate(update):
        for j,y in enumerate(update):
            if i<j and y in E[x]:
                ok = False
    if ok:
        p1 += update[len(update)//2]
    else:
        good = []
        Q = deque([])
        D = {v: len(E[v] & set(update)) for v in update}

        for v in update:
            print(f'E[v]        {E[v]}')
            print(f'set(update) {set(update)}')
            print(f'&           {E[v] & set(update)}')
            print()

        for v in update:
            if D[v] == 0:
                Q.append(v)
        while Q:
            x = Q.popleft()
            good.append(x)
            for y in ER[x]:
                if y in D:
                    D[y] -= 1
                    if D[y] == 0:
                        Q.append(y)
        p2 += good[len(good)//2]
print(p1)
print(p2)

