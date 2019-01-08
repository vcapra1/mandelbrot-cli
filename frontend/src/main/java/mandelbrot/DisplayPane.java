package mandelbrot;

import javafx.scene.layout.StackPane;
import javafx.scene.canvas.Canvas;
import javafx.scene.canvas.GraphicsContext;
import javafx.scene.paint.Color;
import javafx.scene.image.Image;

/**
 * A combination of two Canvases: the one where the rendered image is shown, and 
 * the one where the user can select a region of the image to zoom into.
 */
public class DisplayPane extends StackPane {
    
    /**
     * The two canvases that make up this Pane, and their respective 
     * GraphicsContext objects
     */
    private Canvas mRenderCanvas, mSelectionCanvas;
    private GraphicsContext mRenderGC, mSelectionGC;

    /**
     * The width and height of the pane, which the canvases will fill
     */
    private int mWidth, mHeight;

    /**
     * The width and height of the render, and the top left coordinates
     * such that it is the largest possible that fits in the canvas, and
     * is centered
     */
    private int mRenderTop, mRenderLeft, mRenderWidth, mRenderHeight;

    /**
     * The current setting that defines the selection box (or point)
     */
    private double mBoxLeft, mBoxRight, mBoxTop, mBoxBottom;

    /**
     * The last drawn location of the cursor indicator
     */
    private double mCursorX, mCursorY;

    /**
     * Used when updating the selection to indicate which type of mouse
     * event caused the update
     */
    private enum Setting { POINT, CURSOR, BOX };

    public DisplayPane() {
        this(800, 800);
    }

    public DisplayPane(int width, int height) {
        // Save the dimensions
        mWidth = width;
        mHeight = height;

        // Create the Canvases
        mRenderCanvas = new Canvas(mWidth, mHeight);
        mSelectionCanvas = new Canvas(mWidth, mHeight);

        // Get their Graphics contexts
        mRenderGC = mRenderCanvas.getGraphicsContext2D();
        mSelectionGC = mSelectionCanvas.getGraphicsContext2D();

        // Add listeners to the selection canvas
        initSelectionCanvas();

        // Add the canvases to this pane
        this.getChildren().addAll(mRenderCanvas, mSelectionCanvas);
    }

    /**
     * Set up the selection canvas by adding the event listeners
     */
    private void initSelectionCanvas() {
        // On mouse click, set the center
        mSelectionCanvas.setOnMousePressed((event) -> {
            // Make sure it's enabled
            if (!DisplayPane.this.isDisabled()) {
                // Update the setting
                DisplayPane.this.updateSetting(event.getX(), event.getY(), Setting.POINT);
            }
        });

        // On mouse move, draw the crosshair to follow the cursor
        mSelectionCanvas.setOnMouseMoved((event) -> {
            // Make sure it's enabled
            if (!DisplayPane.this.isDisabled()) {
                // Update the mouse position
                DisplayPane.this.updateSetting(event.getX(), event.getY(), Setting.CURSOR);
            }
        });

        // When the mouss is dragged, update the box
        mSelectionCanvas.setOnMouseDragged((event) -> {
            // Make sure it's enabled
            if (!DisplayPane.this.isDisabled()) {
                // Update the settings
                DisplayPane.this.updateSetting(event.getX(), event.getY(), Setting.BOX);
            }
        });

        // When the mouse leaves, hide the cursor indicator
        mSelectionCanvas.setOnMouseExited((event) -> {
            DisplayPane.this.updateSetting(event.getX(), event.getY(), Setting.CURSOR);
        });
    }

    /**
     * Display the provided image in the render canvas
     */
    public void updateImage(Image image) {
        // Calculate the aspect ratio of both the rendered image and the canvas
        double canvasAspect = (double) mWidth / (double) mHeight;
        double renderAspect = image.getWidth() / image.getHeight();

        // Calculate the bounds of the image
        if (canvasAspect > renderAspect) {
            // The render is taller than the canvas, so restrict the size by height
            mRenderHeight = mHeight;
            mRenderWidth = (int) (mRenderHeight * renderAspect);

            // Calculate the top-left coords to center the image
            mRenderTop = 0;
            mRenderLeft = (int) ((mHeight - mRenderHeight) / 2.0);
        } else {
            // The render is wider than the canvas, so restrict the size by width
            mRenderWidth = mWidth;
            mRenderHeight = (int) (mRenderHeight * renderAspect);

            // Calculate the top-left coords to center the image
            mRenderLeft = 0;
            mRenderLeft = (int) ((mWidth - mRenderWidth) / 2.0);
        }

        // Show the image
        mRenderGC.drawImage(image, mRenderLeft, mRenderTop, mRenderWidth, mRenderHeight);

        // Clear the selection
        updateSetting(-1.0, -1.0, Setting.POINT);
        updateSetting(-1.0, -1.0, Setting.CURSOR);
    }

    /**
     * Update the selection region given the mouse coordinates x and y, and the type
     * of update to apply
     */
    private void updateSetting(double x, double y, Setting type) {
        // Shift the coordinates to account for the location of the image
        x -= mRenderLeft;
        y -= mRenderTop;

        switch (type) {
            case POINT:
                // Reset the box left and right, and set the dimensions to 0
                mBoxLeft = mBoxRight = x;
                mBoxTop = mBoxBottom = y;

                break;
            case CURSOR:
                // Don't change the setting, just update the crosshair that
                // shows where the cursor is
                mCursorX = x;
                mCursorY = y;

                break;
            case BOX:
                // Update the right and bottom coords of the selection box
                mBoxRight = x;
                mBoxBottom = y;

                // Validate the box's bounds
                validateBox();

                break;
        }

        // Re-draw the canvas
        updateSelection();
    }

    /**
     * Makes sure the box's bounds are valid (inside the valid region,
     * and not flipped)
     */
    private void validateBox() {
        // If the box's bounds are incorrect, switch them
        if (mBoxLeft > mBoxRight) {
            double temp = mBoxLeft;
            mBoxLeft = mBoxRight;
            mBoxRight = temp;
        }
        if (mBoxTop > mBoxBottom) {
            double temp = mBoxTop;
            mBoxTop = mBoxBottom;
            mBoxBottom = temp;
        }

        // Ensure the coordinates are inside the image
        if (mBoxLeft < 0) mBoxLeft = 0;
        if (mBoxLeft >= mRenderWidth) mBoxLeft = mRenderWidth - 1;
        if (mBoxRight < 0) mBoxRight = 0;
        if (mBoxRight >= mRenderWidth) mBoxRight = mRenderWidth - 1;
        if (mBoxTop < 0) mBoxTop = 0;
        if (mBoxTop >= mRenderHeight) mBoxTop = mRenderHeight - 1;
        if (mBoxBottom < 0) mBoxBottom = 0;
        if (mBoxBottom >= mRenderHeight) mBoxBottom = mRenderHeight - 1;
    }

    /**
     * Re-draw the selection canvas after a setting has been changed
     */
    private void updateSelection() {
        // Clear the canvas
        mSelectionGC.clearRect(0, 0, mWidth, mHeight);
        
        // If the box's bounds are zero, draw a crosshair at the selection point
        if (mBoxLeft == mBoxRight || mBoxTop == mBoxBottom) {
            
        } else {
            // TODO Draw the box as it was selected with just an outline

            // TODO Draw the box with the set aspect that fits inside the selected region
        }

        // Draw the cursor indicator
        if (mCursorX >= 0.0 && mCursorY >= 0.0) {
            // Use a white stroke
            mSelectionGC.setStroke(Color.WHITE);

            // Draw the two lines
            mSelectionGC.beginPath();
            mSelectionGC.moveTo(0, mCursorY);
            mSelectionGC.lineTo(mWidth - 1, mCursorY);
            mSelectionGC.moveTo(mCursorX, 0);
            mSelectionGC.lineTo(mCursorX, mHeight - 1);
            mSelectionGC.stroke();
        }
    }

}
