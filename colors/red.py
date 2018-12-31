stops = [
    [0, 255, 0, 0],
    [.26, 255, 140, 0],
    [.6, 255, 255, 0],
    [.62, 255, 255, 0],
    [1, 255, 0, 0]
]

data = []

N = 2048

for i in range(N):
    sector = 0
    p = i / N

    while p > stops[sector + 1][0]:
        sector += 1

    inter = (p - stops[sector][0]) / (stops[sector + 1][0] - stops[sector][0])

    r = stops[sector][1] * (1 - inter) + stops[sector + 1][1] * inter
    g = stops[sector][2] * (1 - inter) + stops[sector + 1][2] * inter
    b = stops[sector][3] * (1 - inter) + stops[sector + 1][3] * inter

    print(int(r),",",int(g),",",int(b),sep="")
