from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize
from  graphic_user_interface.windows.secure_shell.file_area import FileArea
from  graphic_user_interface.windows.secure_shell.console import console_menu

class SecunaryMenu:
    def __init__(self,ssh):
        self.ssh=ssh

    def secundary_menu(self) -> QGridLayout:
        layout = QGridLayout()
        btn1 = QPushButton("ana")

        layout.addWidget(btn1, 0, 0)

        btn2 = QPushButton("are")
        layout.addWidget(btn2, 1, 0)

        console_btn = QPushButton("Console")
        console_btn.clicked.connect(self.toggle_console)

        layout.addWidget(console_btn, 2, 0, alignment=Qt.AlignmentFlag.AlignBottom)

        return layout
    def toggle_console(self):
        if self.ssh.console_status:
            if not self.ssh.console_widget:
                self.ssh.console_widget = console_menu(self.ssh)
                self.ssh.layout.addWidget(self.ssh.console_widget, 2, 0)
                self.ssh.layout.setRowStretch(1, 70)
                self.ssh.layout.setRowStretch(2, 25)
        else:
            if self.ssh.console_widget:
                self.ssh.console_widget.setParent(None)
                self.ssh.console_widget.deleteLater()
                self.ssh.console_widget = None
                self.ssh.layout.setRowStretch(1, 95)
                self.ssh.layout.setRowStretch(2, 0)
        self.ssh.console_status = not self.ssh.console_status