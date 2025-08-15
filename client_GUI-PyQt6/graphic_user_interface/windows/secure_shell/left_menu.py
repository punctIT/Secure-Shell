from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize
from  graphic_user_interface.windows.secure_shell.file_area import FileArea


class SecunaryMenu:
    def __init__(self,ssh):
        self.ssh=ssh

    def secundary_menu(self) -> QGridLayout:
        layout = QGridLayout()
        btn1 = QPushButton("ana")

        layout.addWidget(btn1, 0, 0)

        btn2 = QPushButton("are")
        layout.addWidget(btn2, 1, 0)

        btn3 = QPushButton("mere")
        layout.addWidget(btn3, 2, 0)

        return layout
