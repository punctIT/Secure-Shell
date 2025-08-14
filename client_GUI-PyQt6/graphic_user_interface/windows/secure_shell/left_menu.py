from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize
from file_area import get_files_area

def secundary_menu(self)->QGridLayout:
    layout = QGridLayout()
    btn1=QPushButton("ana")
    btn1.clicked.connect(lambda : print("working"))
    layout.addWidget(btn1,0,0)

    btn2 = QPushButton("are")
    layout.addWidget(btn2, 1, 0)
    btn2.clicked.connect(lambda: print("working"))

    btn2 = QPushButton("mere")
    layout.addWidget(btn2, 2, 0)
    btn2.clicked.connect(lambda: print("working"))

    return  layout