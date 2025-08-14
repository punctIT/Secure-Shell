from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout, QWidgetAction, QFileDialog
)
from PyQt6.QtGui import QCursor
from PyQt6.QtCore import Qt

class LoginWindow(QWidget):
    def __init__(self, parent_window=None):
        super().__init__()
        self.parent=parent_window
        print(self.parent.client.receive4096())
        self.layout = QGridLayout()
        self.layout.setContentsMargins(50, 50, 50, 50)

        self.signin_label = QLabel("Welcome back")
        self.signin_label.setObjectName("Tlabel")
        self.signin_label.setContentsMargins(1, 1, 1, 50)
        self.signin_label.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.layout.addWidget(self.signin_label, 0, 0)

        self.username_label = QLabel("Username ")
        self.layout.addWidget(self.username_label, 1, 0)
        self.username_input = QLineEdit()
        self.username_input.setContentsMargins(3, 1, 1, 10)
        self.layout.addWidget(self.username_input, 2, 0)

        sub_grid = QGridLayout()
        sub_grid.setSpacing(0)
        sub_grid.setContentsMargins(3, 1, 1, 10)
        self.password_label = QLabel("Password ")
        self.layout.addWidget(self.password_label, 3, 0)

        self.password_input = QLineEdit()
        self.password_input.setEchoMode(QLineEdit.EchoMode.Password)
        sub_grid.addWidget(self.password_input, 0, 0, 1, 1)

        self.secret=True
        self.show_password_btn = QPushButton("👁")
        self.show_password_btn.setObjectName("showbtn")
        self.show_password_btn.setCursor(QCursor(Qt.CursorShape.PointingHandCursor))
        self.show_password_btn.clicked.connect(self.show_password)
        sub_grid.addWidget(self.show_password_btn, 0, 1, 1, 1)
        self.show_password_btn.setFixedHeight(self.password_input.sizeHint().height())

        sub_grid.setColumnStretch(0, 99)
        sub_grid.setColumnStretch(1, 1)
        self.layout.addLayout(sub_grid, 4, 0)


        self.signin_btn=QPushButton("Sign in")
        self.signin_btn.clicked.connect(self.login_action)
        self.signin_btn.setCursor(QCursor(Qt.CursorShape.PointingHandCursor))
        self.layout.addWidget(self.signin_btn, 5, 0)

        with open("graphic_user_interface/styles/login_window.css") as file:
            self.setStyleSheet(file.read())

        self.setLayout(self.layout)
    def show_password(self):
        if self.secret is True:
            self.password_input.setEchoMode(QLineEdit.EchoMode.Normal)
            self.secret=False
        else:
            self.password_input.setEchoMode(QLineEdit.EchoMode.Password)
            self.secret=True
    def login_action(self):
        self.username_input.setStyleSheet("border: 2px solid;")
        self.password_input.setStyleSheet("border: 2px solid;")
        login="login "+self.username_input.text()+" "+self.password_input.text()
        self.parent.client.sent(login)
        message=self.parent.client.receive()
        stript_message=message.strip()
        if stript_message == "?&NSuccesful login[-]:[-]":
            self.parent.show_secure_shell_window()
        else:
            self.username_input.setStyleSheet("border: 2px solid rgb(50, 128, 142);")
            self.password_input.setStyleSheet("border: 2px solid rgb(50, 128, 142);")










