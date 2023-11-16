import matplotlib.pyplot as plt
import numpy as np

# Define the function for xn+1
def logistic_map(x, c):
    return x**2 + c

# Set the initial values
c = -1  # Change this to your desired value of 'c'

# Initialize a list to store the (x, x_next) pairs
x_array = [0.25, -0.9375, -0.12109375, -0.9853363037109375, -0.029112368589267135, -0.9991524699951226, -0.0016943417026455965, -0.9999971292061947, -5.741579369278327e-6, -0.9999999999670343, -6.593148249578462e-11, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0]


# Initialize the starting point
x_start, y_start = x_array[0], x_array[0]

for i in range(1, len(x_array)):
    y = x_array[i]
    
    plt.arrow(x_start, y_start, 0, y - y_start, head_width=0.0015, head_length=0.002, fc='blue', ec='blue', lw=1.5)
    x_start, y_start = x_start, y

    plt.arrow(x_start, y_start, y_start - x_start, 0, head_width=0.0015, head_length=0.002, fc='red', ec='red', lw=1.5)
    x_start, y_start = y_start, y_start

# Set the x-axis and y-axis limits
x_min = min(x_array) - 0.1
x_max = max(x_array) + 0.1
y_min = min(x_array) - 0.1
y_max = max(x_array) + 0.1
plt.xlim(x_min, x_max)
plt.ylim(y_min, y_max)

x_identity = np.linspace(x_min, x_max, 100)
plt.plot(x_identity, x_identity, linestyle='--', color='green', label='y = x')

x_square_plus_c = np.linspace(x_min, x_max, 100)
y_square_plus_c = x_square_plus_c**2 + c
plt.plot(x_square_plus_c, y_square_plus_c, linestyle='--', color='purple', label='y = x^2 + c')

plt.legend()
plt.show()

