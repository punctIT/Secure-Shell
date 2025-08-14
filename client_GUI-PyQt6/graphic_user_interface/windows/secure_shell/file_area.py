from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)
from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize

def get_files_area(self,test_text) -> QScrollArea:
    scroll_area = QScrollArea()
    scroll_area.setWidgetResizable(True)

    container = QWidget()
    scroll_area.setWidget(container)
    scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
    scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)

    scroll_grid = QGridLayout(container)

    row = 0
    col = 0
    files = test_text[3:].split("\n\n")
    for i in files:
        if i.startswith("^!"):
            i = i[2:]
            file = QGridLayout()
            file.setContentsMargins(10, 10, 10, 10)

            btn = QPushButton()
            btn.clicked.connect(lambda _, val=i: print(val))
            btn.setIcon(QIcon("../Assets/Icons/folder.png"))  # modifiy
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
        if not i.startswith("^!"):
            file = QGridLayout()
            file.setContentsMargins(10, 10, 10, 10)

            btn = QPushButton()
            if not i.startswith("^#"):
                btn.setIcon(QIcon("../Assets/Icons/file.png"))  # modifiy
            else:
                i = i[2:]
                btn.setIcon(QIcon("../Assets/Icons/exe.png"))  # modifiy
            btn.setIconSize(QSize(64, 64))
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
