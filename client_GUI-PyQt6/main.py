import sys
from PyQt6.QtWidgets import QApplication
from graphic_user_interface.client_gui import Window

def main():
    app = QApplication(sys.argv)
    window = Window()
    window.show()
    sys.exit(app.exec())

if __name__ == "__main__":
    main()