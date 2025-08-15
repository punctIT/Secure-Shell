from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize


def update_file_area(self):
    item = self.primary_layout.itemAtPosition(0, 1)
    if item:
        widget = item.widget()
        if widget:
            self.primary_layout.removeWidget(widget)
            widget.setParent(None)
            widget.deleteLater()
    new_area = get_files_area(self)
    self.primary_layout.addWidget(new_area, 0, 1)


def get_files_area(self) -> QScrollArea:
    scroll_area = QScrollArea()
    scroll_area.setWidgetResizable(True)

    container = QWidget()
    scroll_area.setWidget(container)
    scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
    scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)

    scroll_grid = QGridLayout(container)

    row = 0
    col = 0
    files = self.files[3:].split("\n\n")
    for i in files:
        if len(i)==0:
            continue
        if i.startswith("^!"):
            i = i[2:]
            file = QGridLayout()
            file.setContentsMargins(10, 10, 10, 10)

            btn = QPushButton()
            btn.clicked.connect(lambda _, val=i: print(val))

            btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/folder.png"))
            btn.setIconSize(QSize(64, 64))
            btn.setContextMenuPolicy(Qt.ContextMenuPolicy.CustomContextMenu)
            btn.customContextMenuRequested.connect(lambda pos, b=btn: show_context_menu(self,pos, b))
            file.addWidget(btn, 0, 0)

            name = QLabel(i)
            name.setAlignment(Qt.AlignmentFlag.AlignHCenter | Qt.AlignmentFlag.AlignTop)
            name.setWordWrap(True)
            file.addWidget(name, 1, 0)

            scroll_grid.addLayout(file, row, col)
            col = (col + 1) % 4
            if col == 0:
                row += 1
    for i in files:
        if len(i)==0:
            continue
        if not i.startswith("^!"):
            file = QGridLayout()
            file.setContentsMargins(10, 10, 10, 10)

            btn = QPushButton()
            if not i.startswith("^#"):
                btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/file.png"))
            else:
                i = i[2:]
                btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/exe.png"))
            btn.setIconSize(QSize(64, 64))
            btn.clicked.connect(lambda _, val=i: print(val))
            file.addWidget(btn, 0, 0)

            name = QLabel(i)
            name.setAlignment(Qt.AlignmentFlag.AlignHCenter | Qt.AlignmentFlag.AlignTop)
            name.setWordWrap(True)
            file.addWidget(name, 1, 0)

            scroll_grid.addLayout(file, row, col)
            col = (col + 1) % 4
            if col == 0:
                row += 1
    return scroll_area

def show_context_menu(self, pos,btn):
    menu = QMenu()
    menu.addAction("Opțiunea 1", lambda: print("Opțiunea 1 selectată"))
    menu.addAction("Opțiunea 2", lambda: print("Opțiunea 2 selectată"))
    menu.addSeparator()
    menu.addAction("Ieșire", self.close)

    # afișăm meniul la poziția cursorului
    menu.exec(btn.mapToGlobal(pos))
