from PyQt6.QtWidgets import QWidget, QVBoxLayout
from graphic_user_interface.windows.connect_window import ConnectWindow
from graphic_user_interface.windows.login_window import LoginWindow
from graphic_user_interface.windows.secure_shell.secure_shell_window import SecureShellWindow
from backend.client import TlsClient

class Window(QWidget):
    def __init__(self):
        super().__init__()
        self.client=TlsClient()
        self.setAutoFillBackground(True)
        self.setWindowTitle("Secure Shell")
        self.main_layout = QVBoxLayout()
        self.current_widget = None
        self.show_connect_window()

        self.setLayout(self.main_layout)


    def clear_and_add_widget(self, new_widget):
        if self.current_widget:
            self.main_layout.removeWidget(self.current_widget)
            self.current_widget.deleteLater()

        self.current_widget = new_widget
        self.main_layout.addWidget(self.current_widget)

    def show_connect_window(self):
        connect_window = ConnectWindow(self)
        self.clear_and_add_widget(connect_window)

    def show_login_window(self):
        login_window = LoginWindow(self)
        self.clear_and_add_widget(login_window)
    def show_secure_shell_window(self):
        ssh_window= SecureShellWindow(self)
        self.clear_and_add_widget(ssh_window)