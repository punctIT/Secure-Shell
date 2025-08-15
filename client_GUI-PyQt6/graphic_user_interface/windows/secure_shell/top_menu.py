from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)

from  graphic_user_interface.windows.secure_shell.console import console_menu
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize


class PrimaryMenu:
    def __init__(self,ssh):
        self.ssh=ssh
        self.path_label = QLabel("")
    def primary_menu(self) -> QGridLayout:
        layout = QGridLayout()

        console_btn = QPushButton("Console")
        console_btn.clicked.connect(self.toggle_console)
        layout.addWidget(console_btn, 0, 3)

        back_btn = QPushButton()
        back_btn.clicked.connect(self.back_function)
        back_btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/backward.png"))
        back_btn.setIconSize(QSize(20, 20))
        layout.addWidget(back_btn, 0, 0)

        forward_btn = QPushButton()
        forward_btn.clicked.connect(self.forward_funtion)
        forward_btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/forward.png"))
        forward_btn.setIconSize(QSize(20, 20))
        layout.addWidget(forward_btn, 0, 1)

        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)

        container = QWidget()
        scroll_area.setWidget(container)
        scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)
        scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        scroll_grid = QGridLayout(container)
        self.path_label = QLabel(f"Server{self.ssh.current_path}")
        scroll_grid.addWidget(self.path_label, 0, 0)
        layout.addWidget(scroll_area, 0, 2)
        return layout
    def update_path_label(self):
        self.path_label.setText(f"Server{self.ssh.current_path}")
        print("not working")
    def back_function(self):
        self.ssh.file_area.folder_function("..")
        #self.update_path_label()
    def forward_funtion(self):
        print("forward")

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




