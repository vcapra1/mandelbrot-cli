# Mandelbrot Set Explorer

This is a CLI for generation Mandelbrot fractals.  It uses the Nvidia CUDA platform to run the computations on an Nvidia graphics card, and therefore requires an Nvidia GPU to run.

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
