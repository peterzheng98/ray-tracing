import imageio
import sys
import numpy as np
from PIL import Image


assert len(sys.argv) == 2, "Require a ppm type file"
r = open(sys.argv[1]).readlines()
r.remove('\n') if '\n' in r else None
r = [i.strip('\n') for i in r]
assert 'P3' == r[0], "Not PPM P3 files"
size_x, size_y = r[1].split(' ')
size_x = int(size_x)
size_y = int(size_y)
max_data = int(r[2])
# currently we only support rgb with 255
assert max_data == 255, "Not Implemented"
data_raw = np.zeros((size_x, size_y, 3))
r = r[3:]
for idx, d in enumerate(r):
    r, g, b = d.split(' ')
    r = int(r)
    g = int(g)
    b = int(b)
    data_raw[idx // size_x, idx % size_x] = np.array([r, g, b])
print('Graph generated with size: ', data_raw.shape)
image = Image.fromarray(data_raw.astype('uint8')).convert('RGB')
image.save('demo.png')
