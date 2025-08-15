from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)

from  graphic_user_interface.windows.secure_shell.console import console_menu
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize

def primary_menu(self)->QGridLayout:
    layout = QGridLayout()

    console_btn = QPushButton("Console")
    console_btn.clicked.connect(lambda: toggle_console(self))
    layout.addWidget(console_btn, 0, 2)

    back_btn = QPushButton()
    back_btn.clicked.connect(lambda:back_function(self))
    back_btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/backward.png"))
    back_btn.setIconSize(QSize(20, 20))
    layout.addWidget(back_btn, 0, 0)

    forward_btn = QPushButton()
    forward_btn.clicked.connect(lambda: forward_btn(self))
    forward_btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/forward.png"))
    forward_btn.setIconSize(QSize(20, 20                                                                                                ))
    layout.addWidget(forward_btn, 0, 1)

    return layout
def back_function(self):
    print("back")
def forward_function(self):
    print("forward")
def toggle_console(self):
    if self.console_status:
        if not self.console_widget:
            self.console_widget = console_menu(self)
            self.layout.addWidget(self.console_widget, 2, 0)
            self.layout.setRowStretch(1, 70)
            self.layout.setRowStretch(2, 25)
    else:
        if self.console_widget:
            self.console_widget.setParent(None)
            self.console_widget.deleteLater()
            self.console_widget = None
            self.layout.setRowStretch(1, 95)
            self.layout.setRowStretch(2, 0)
    self.console_status = not self.console_status
