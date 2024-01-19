import matplotlib.pyplot as plt
import numpy as np
from matplotlib.widgets import Slider

fig = plt.figure()
ax = plt.axes(projection="3d")


def plot_line(p, v, max_val):
    px = p[0] + v[0] * np.linspace(0, max_val, 2)
    py = p[1] + v[1] * np.linspace(0, max_val, 2)
    pz = p[2] + v[2] * np.linspace(0, max_val, 2)

    return ax.plot(px, py, pz)


def update(max_val):
    global ax
    ax.clear()

    with open("./src/inputs/input.txt") as f:
        for i, line in enumerate(f.readlines()):
            # if i >= 2:
            #     break

            (pos, vel) = line.split(" @ ")
            pos = np.array([int(x) for x in pos.split(", ")])
            vel = np.array([int(x) for x in vel.split(", ")])

            plot_line(pos, vel, max_val)

    pos = [58811373499876, 365938348079364, 103399838012044]
    vel = [554, -54, 242]
    plot_line(pos, vel, max_val)


# axslider = plt.axes((0.25, 0.1, 0.65, 0.03))
# slider = Slider(axslider, "Slider", 0, 1000_000_000_000)
# slider.on_changed(update)
# update(1_000_000_000_000)
update(1_000_000_000_000)

plt.show()
