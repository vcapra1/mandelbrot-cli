# Mandelbrot Set Explorer

## Problems
* User can draw box outside of image in GUI
* Re-rendering even when config is the same

## To-do
* Add progress messages to the GUI, such as "preparing" while the render
memory is being allocated, and "saving image" while the image is being
exported, as both tend to take more than a few seconds when the image is
sufficiently large.
* Don't ever clone a Render struct, because it holds ALL the pixel data,
which is a ton of memory, and images larger than about 2k pixels are
exhausting the memory and crashing the program.
* In the GUI, add a feature to save the image (which will just require
copying the exported file from the /tmp directory to a user-decided
location
* Maybe add a "Cancel" button while the render is in progress, which
halts the device threads, or whatever is happening at that time
* Make window non-resizable
* Package frontend and backend into one program

## Workflow of using the program
There is the notion of a `Render`, which is essentially a 2-dimensional array of tuples, each corresponding to a pixel and containing the following information:
* The complex coordinate corresponding to the pixel (c)
* The complex value z, calculated at the current iteration
* The number of iterations that this pixel has been computed for, and
* Whether or not the pixel has diverged

The `Render` structure also stores information about the image's size and number of iterations calculated so far.

By storing the `Render` in this format, we can go back and increase the number of iterations without needing to recalculate up to the current number of iterations.
For example, if the `Render` has been computed up to 2000 iterations, a `Render` of the same window reigon can be computed by picking up from iteration 2000 only
on the pixels that have not yet diverged, which will be much faster than re-calculating all 3000 iterations.

This also allows for very fast color adjustments, because the color only depends on the diverged value and number of iterations.  Therefore, adjusting the color
function merely requires re-computing the color for each coordinate, and not doing any iterations.

The colorized image is referred to as an `Image` construct.  This, like a `Render`, is a 2-dimensional array, but is instead an array of Colors.  The `Image` also
contains information about the image's size.

With this setup, the task of exporting the image merely comes down to applying the color function to the `Render` to get an `Image`, and then saving the color data
to a PNG file.

## Parameters
In order to render, the following parameters must be specified:
* Complex center
* Radius (min if width != height)
* Image size (width, height)
* Max number of iterations
* Supersampling factor

And to generate the image, the render is passed through a color function which takes the `Render`, number of iterations and diverged coordinate for each pixel, and
the max number of iterations for the image, and an `Image` is outputted.  For the standard color function, the function is defined by the color shift and scale.
This may later be generalized to define more complex and customizable color functions, another advantage of separating the rendering and colorizing steps.

## Supersampling
Supersampling will be accomplished by rendering the image at an integer multiple larger than specified, and then scaling the image back down when exporting, using
an appropriate interpolation method.

## Arithmetic
Across the entire project, a special type of floating-point number will be used for coordinates, radii, and calculations.  Initially, this type will just be an alias
of `f64`, but later I hope to be able to write or use a new type that allows for multiple-precision arithmetic, so we can zoom much further into the image.

## Back-end
The backend will be written in Rust, using FFI with C to run CUDA computations.  The only CUDA code will be the iterative function z(n + 1) = z(n) ^ 2 + c.  All
other computations, such as the color function, will be done in Rust because they are not nearly as computationally intensive.  The actual computation is
O(n^2\*m), where n is the image dimension and m is the max number of iterations.  The color function should be O(n^2), which is much smaller and doesn't involve
high-precision floating-point computation.

## Front-end
The front end for this program will be written in Java, as it is easy to write a GUI in Java that is cross-platform.  The Java program and the Rust program will
communicate via Sockets on a high port.  It will feature a large image display of the current rendered image, allowing users to change parameters directly via
numerical inputs, or drawing a box on the image to zoom into or out of.

The program will also work purely from the command line, by simply running the Rust program and omitting the GUI flag.
