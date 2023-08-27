from matplotlib import pyplot as plt

import matplotlib.animation as animation


with open("results.txt", "r") as ofile:
    data = ofile.readlines()
data = [[float(y) for y in x.strip().split(",")] for x in data]


img = []  # some array of images
frames = []  # for storing the generated images
fig = plt.figure(figsize=(5, 5))
for idx, line in enumerate(data):
    pt_a_x = line[0]
    pt_a_y = line[1]
    pt_b_x = line[6]
    pt_b_y = line[7]
    pt_c_x = line[12]
    pt_c_y = line[13]
    frames.append(
        plt.plot([pt_a_x, pt_b_x], [pt_a_y, pt_b_y], "r", [pt_a_x, pt_c_x], [pt_a_y, pt_c_y], "g", [pt_c_x, pt_b_x], [pt_c_y, pt_b_y], "b"))

ani = animation.ArtistAnimation(fig, frames, interval=50, blit=True)
ani.save('movie.mp4')
