from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout, QFileDialog
)

class LoginWindow(QWidget):
    def __init__(self, parent_window=None):
        super().__init__()
        self.layout = QGridLayout()
        self.label1 = QLabel("Login: ")
        self.layout.addWidget(self.label1, 0, 0)
        self.username_edit = QLineEdit()
        self.layout.addWidget(self.username_edit, 0, 1)


        self.setLayout(self.layout)


    def get_connect_page_layout(self):
        return self.layout