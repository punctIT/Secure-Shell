from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)

from graphic_user_interface.windows.secure_shell.file_area import FileArea
from  graphic_user_interface.windows.secure_shell.left_menu import SecunaryMenu
from  graphic_user_interface.windows.secure_shell.top_menu import PrimaryMenu


class SecureShellWindow(QWidget):
    def __init__(self, parent_window=None):
        super().__init__()
        self.adjustSize()

        self.parent=parent_window
        self.file_area=FileArea(self)
        self.primary_menu=PrimaryMenu(self)
        self.secundary_menu=SecunaryMenu(self)

        self.current_path=None
        self.files=None
        self.stack = []
        self.update_path()

        self.layout = QGridLayout()
        self.setLayout(self.layout)
        self.layout.setRowStretch(0, 5)
        self.layout.setRowStretch(1, 95)

        self.layout.addLayout(self.primary_menu.primary_menu(),0,0)

        self.primary_layout=QGridLayout()
        self.console_status=True
        self.console_widget = None

        self.primary_layout.addLayout(self.secundary_menu.secundary_menu(),0,0)
        self.primary_layout.addWidget(self.file_area.get_files_area(),0,1)
        self.primary_layout.setColumnStretch(0, 10)
        self.primary_layout.setColumnStretch(1, 90)
        self.layout.addLayout(self.primary_layout,1,0)

        with open("graphic_user_interface/styles/ssh_window.css") as file:
            self.setStyleSheet(file.read())
    def update_path(self):
        self.parent.client.sent("ls")
        text=self.parent.client.receive().strip()
        output=text.split("[-]")

        self.current_path=output[1]
        self.files=output[0]

