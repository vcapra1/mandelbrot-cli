package mandelbrot;

import javafx.application.Application;
import javafx.stage.Stage;
import javafx.scene.Scene;

import javafx.scene.layout.VBox;
import javafx.scene.*;
import javafx.scene.paint.*;
import javafx.scene.canvas.*;

public class App extends Application {

    // The canvas that the render will be drawn on and the user
    // will interact with to select a window to zoom.
    private Canvas mRenderCanvas;
    private GraphicsContext mRenderGC;

    public static void main(String[] args) {
        try {
            // Connect to the backend
            SocketComm socketComm = new SocketComm(Integer.parseInt(args[0]));

            // Launch the application
            launch(args);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    @Override
    public void start(Stage primaryStage) {
        // Create the canvas
        Canvas canvas = new Canvas(800, 800);
        GraphicsContext gc = canvas.getGraphicsContext2D();

        // TODO: create the controls

        // Create the controls layout
        ControlPane controlsPane = new ControlPane();

        // Create the root layout
        VBox root = new VBox(canvas, controlsPane);

        // Create and apply the scene
        Scene scene = new Scene(root, 800, 950);
        primaryStage.setScene(scene);

        // Show the window
        primaryStage.show();

        // Disable resizing the stage
        //primaryStage.setResizable(false);
    }

}
