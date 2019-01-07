package mandelbrot;

import javafx.application.Application;
import javafx.stage.Stage;
import javafx.scene.Scene;

import javafx.scene.layout.VBox;
import javafx.scene.layout.StackPane;
import javafx.scene.*;
import javafx.scene.paint.*;
import javafx.scene.canvas.*;


public class App extends Application {

    private GraphicsContext mGcDraw;
    private SocketComm mSocketComm;
    private ControlPane mControlsPane;

    private static String[] sArgs;

    private int mDragStartX = 0, mDragStartY = 0,
        setX = -1, setY = -1, setW = 0, setH = 0;

    public static void main(String[] args) {
        sArgs = args;

        // Launch the application
        launch(args);
    }

    @Override
    public void start(Stage primaryStage) {
        try {
            // Connect to the backend
            mSocketComm = new SocketComm(Integer.parseInt(sArgs[0]));
        } catch (Exception e) {
            e.printStackTrace();
            return;
        }

        // Create the canvas for the image
        Canvas canvas = new Canvas(800, 800);
        GraphicsContext gc = canvas.getGraphicsContext2D();

        Canvas drawCanvas = new Canvas(800, 800);
        GraphicsContext gcDraw = drawCanvas.getGraphicsContext2D();
        mGcDraw = gcDraw;

        drawCanvas.setOnMousePressed((event) -> {
            if (mControlsPane.isEnabled()) {
                gcDraw.clearRect(0, 0, 800, 800);

                setX = (int) event.getX();
                setY = (int) event.getY();
                setW = setH = 0;

                drawSetting(gcDraw);
            }
        });

        drawCanvas.setOnMouseMoved((event) -> {
            if (mControlsPane.isEnabled()) {
                gcDraw.clearRect(0, 0, 800, 800);

                gcDraw.setStroke(Color.WHITE);
                gcDraw.beginPath();
                gcDraw.moveTo(0, event.getY());
                gcDraw.lineTo(800, event.getY());
                gcDraw.moveTo(event.getX(), 0);
                gcDraw.lineTo(event.getX(), 800);
                gcDraw.stroke();

                mDragStartX = (int) event.getX();
                mDragStartY = (int) event.getY();

                // Draw current setting
                drawSetting(gcDraw);
            }
        });

        drawCanvas.setOnMouseExited((event) -> {
            if (mControlsPane.isEnabled()) {
                gcDraw.clearRect(0, 0, 800, 800);

                drawSetting(gcDraw);
            }
        });

        drawCanvas.setOnMouseDragged((event) -> {
            if (mControlsPane.isEnabled()) {
                gcDraw.clearRect(0, 0, 800, 800);
                
                gcDraw.setFill(Color.rgb(255, 255, 255, .5));
                int x1 = mDragStartX, y1 = mDragStartY, x2 = (int) event.getX(), y2 = (int) event.getY();
                int left = Math.min(x1, x2);
                int top = Math.min(y1, y2);
                int width = Math.abs(x2 - x1);
                int height = Math.abs(y2 - y1);

                setX = left;
                setY = top;
                setW = width;
                setH = height;

                drawSetting(gcDraw);
            }
        });

        // Create the stackPane for the canvases
        StackPane canvases = new StackPane(canvas, drawCanvas);

        // Create the controls layout
        mControlsPane = new ControlPane(mSocketComm, gc, this);

        // Create the root layout
        VBox root = new VBox(canvases, mControlsPane);

        // Create and apply the scene
        Scene scene = new Scene(root, 800, 950);
        primaryStage.setScene(scene);

        // Show the window
        primaryStage.show();

        // Disable resizing the stage
        //primaryStage.setResizable(false);
    }

    public void resetDrawCanvas() {
        mGcDraw.clearRect(0, 0, 800, 800);

        setX = setY = -1;
        setW = setH = 0;

        drawSetting(mGcDraw);
    }

    private void drawSetting(GraphicsContext gcDraw) {
        if ((setX == -1 || setY == -1) && (setW == 0 && setH == 0)) return;

        // TODO Make sure rectangle is in bounds of the drawing

        if (setW == 0 || setH == 0) {
            // draw crosshair at set x,y
            gcDraw.setStroke(Color.WHITE);
            gcDraw.beginPath();
            gcDraw.moveTo(0, setY);
            gcDraw.lineTo(800, setY);
            gcDraw.moveTo(setX, 0);
            gcDraw.lineTo(setX, 800);
            gcDraw.stroke();
        } else {
            // draw box
            double aspect = mControlsPane.getSavedAspect();
            double drawnAspect = (double) setW / (double) setH;

            gcDraw.setFill(Color.rgb(255, 255, 255, 0.5));
            gcDraw.setStroke(Color.WHITE);
            gcDraw.strokeRect(setX, setY, setW, setH);

            if (aspect < drawnAspect) {
                // taller than drawn box
                double left = setX + 0.5 * (drawnAspect - aspect) * setH;
                gcDraw.fillRect(left, setY, setW * aspect / drawnAspect, setH);
            } else {
                // wider than drawn box
                double top = setY + 0.5 * (1.0 / drawnAspect - 1.0 / aspect) * setW;
                gcDraw.fillRect(setX, top, setW, setH / aspect * drawnAspect);
            }
            
        }
    }

    @Override
    public void stop() {
        try {
            // Close the socket and exit smoothly
            mSocketComm.close();
        } catch (Exception e) {}
    }

}
