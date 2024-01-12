import matplotlib.pyplot as plt
import numpy as np

data = ("16 x 16", "10 000 x 10 000", "50 000 x 50 000", " 100 000 x 100 000", "300 000 x 300 000", "500 000 x 500 000")
types = {
    'without selection': ( 3.841e-6, 0.000664538, 0.003542198,  0.010373978, 0.021704988,0.036349676),
    'with selection': (1.0896e-5, 0.008101561, 0.031014656,0.090381496, 0.210132405, 0.367273693),
}

x = np.arange(len(data))  # the label locations
width = 0.25  # the width of the bars
multiplier = 0

fig, ax = plt.subplots(layout='constrained')

for attribute, measurement in types.items():
    offset = width * multiplier
    rects = ax.bar(x + offset, measurement, width, label=attribute)
    multiplier += 1

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('seconds')
ax.set_title('Speed for A = LU with LUx = b')
ax.set_xticks(x + width, data)
ax.legend(loc='upper left', ncols=2)

plt.show()