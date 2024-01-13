# TODO:
- [x] Write a compare images program
- [x] Learn how to do:
    - [x] low-pass filter
    - [x] high-pass filter
    - [x] unequal quantisers
- [x] Wirte loops that will zig-zag through the image
    - You can change zig-zag to S pattern, it will be easier to code
- [x] Store low-pass and high-pass filters for every color
    - [x] Write library that will contain structer to represent filters
        - [x] function to update numbers in a filter
        - [x] function to get diffrences
- [x] use LGB algorithm to find optimal quantiser for every color and every filter
- [x] quantise every color and every filter, be careful to correctly check, so that errors will not grow
- [x] Save data that is needed to decode a image
- [ ] Decode image

# Notes:
- When coding diffrences, compare them to what decoder would see, to minimize the error. Which in other case would propagate a lot.
- Lecture 10 was about it
    - DPCM
    - Don't go from left to right, but in zig-zag pattern to avoid big changes
        1   2   6   7
        3   5   8   13
        4   9   12  14
        10  11  15  16
    - low-pass filter: y_n = (x_n + x_n-1) / 2
    - high-pass filter: z_n = (x_n - x_n-1) / 2
    - We could send all y_n and z_n, but that would double the size of the image, insted we send y_2n and z_2n, so we can compute x_2n = y_2n + z_2n, and x_2n+1 = y_2n - z_2n
    - y_n should be encoded as diffrences, and x_n as normal, remeber to treat diffrently first element of y_n, x_n as they will not have prior element
    - we will quantise diffrences of y_n and values of z_n
- Use lbg algorithm to find optimal quantiser
