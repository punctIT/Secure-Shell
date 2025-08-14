from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize

def console_menu(self)->QGridLayout:
    layout=QGridLayout()
    console_label = QLabel("text")
    console_label.setAlignment(Qt.AlignmentFlag.AlignHCenter | Qt.AlignmentFlag.AlignTop)
    layout.addWidget(console_label,0,0)
    return layout