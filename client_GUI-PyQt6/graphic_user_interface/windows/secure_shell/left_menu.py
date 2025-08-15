from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize
from  graphic_user_interface.windows.secure_shell.file_area import update_file_area

def secundary_menu(self)->QGridLayout:
    layout = QGridLayout()
    btn1=QPushButton("ana")
    btn1.clicked.connect(lambda :update_file_area(self,"?&L20240824_150239.jpg"))
    layout.addWidget(btn1,0,0)

    btn2 = QPushButton("are")
    layout.addWidget(btn2, 1, 0)
    btn2.clicked.connect(lambda :update_file_area(self,"?&L\n\n^!1\n\n^#1.exe\n\n^!12\n\n^!13\n\n20240824_150239.jpg"))

    btn3 = QPushButton("mere")
    layout.addWidget(btn3, 2, 0)
    btn3.clicked.connect(lambda :update_file_area(self,"?&L\n\n^!1\n\n^#1.exe"))

    return  layout