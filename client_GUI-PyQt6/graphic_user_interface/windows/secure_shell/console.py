from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize

def console_menu(self)-> QScrollArea:
    scroll_area = QScrollArea()
    scroll_area.setWidgetResizable(True)

    container = QWidget()
    scroll_area.setWidget(container)
    scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
    scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)

    scroll_grid = QGridLayout(container)

    for i in range(1,100):
        scroll_grid.addWidget(QLabel(f"text{i}"),i,0)

    return scroll_area
