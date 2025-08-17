from PyQt6.QtWidgets import (
    QWidget,  QLabel, QGridLayout,QScrollArea, QFileDialog, QMenu,  QLineEdit, QWidgetAction, QPushButton
)
from graphic_user_interface.windows.secure_shell.content_menu import Content
from PyQt6.QtGui import QIcon,QCursor,QAction
from PyQt6.QtCore import Qt,QSize

class FileArea:
    def __init__(self,ssh):
        self.ssh=ssh
        self.content = Content(ssh)
    def update_file_area(self):
        item = self.ssh.primary_layout.itemAtPosition(0, 1)
        if item:
            widget = item.widget()
            if widget:
                self.ssh.primary_layout.removeWidget(widget)
                widget.setParent(None)
                widget.deleteLater()
        new_area = self.get_files_area()
        self.ssh.primary_layout.addWidget(new_area, 0, 1)

    def folder_function(self, folder_name):
        command = "cd " + f"\"{folder_name}\""
        self.ssh.parent.client.sent(command)
        print(self.ssh.parent.client.receive())
        self.ssh.update_path()
        self.update_file_area()
        self.ssh.primary_menu.update_path_label()

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
        files = self.ssh.files[3:].split("\n\n")
        for i in files:
            if len(i) == 0:
                continue
            if i.startswith("^!"):
                i = i[2:]
                file = QGridLayout()
                file.setContentsMargins(10, 10, 10, 10)

                btn = QPushButton()
                btn.clicked.connect(lambda _, val=i: self.folder_function(val))

                btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/folder.png"))
                btn.setIconSize(QSize(64, 64))
                btn.setContextMenuPolicy(Qt.ContextMenuPolicy.CustomContextMenu)
                btn.setCursor(QCursor(Qt.CursorShape.PointingHandCursor))
                btn.customContextMenuRequested.connect(lambda pos, b=btn,val=i: self.show_context_menu_dir(pos, b,val))
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
            if len(i) == 0:
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
                btn.clicked.connect(lambda _, val=i: self.content.toggle_content_menu(val))
                btn.setContextMenuPolicy(Qt.ContextMenuPolicy.CustomContextMenu)
                btn.customContextMenuRequested.connect(lambda pos, b=btn, val=i: self.show_context_menu_file(pos, b, val))
                btn.setCursor(QCursor(Qt.CursorShape.PointingHandCursor))
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
    def delete_dir(self,name):
        self.ssh.parent.client.sent(f"rmdir \"{name}\"")
        print(self.ssh.parent.client.receive())
        self.ssh.update_path()
        self.update_file_area()
    def delete_file(self,name):
        self.ssh.parent.client.sent(f"rm \"{name}\"")
        print(self.ssh.parent.client.receive())
        self.ssh.update_path()
        self.update_file_area()
    def rename_menu(self, pos, btn,name):
        def rename(new_name):
            self.ssh.parent.client.sent(f"mv \"{name}\" \"{new_name}\"")
            print(self.ssh.parent.client.receive())
            self.ssh.update_path()
            self.update_file_area()
        menu = QMenu()
        line_edit = QLineEdit()
        line_edit.setText(name)
        line_action = QWidgetAction(menu)
        line_action.setDefaultWidget(line_edit)
        menu.addAction(line_action)

        action_rename = QAction('Rename', menu)
        action_rename.triggered.connect(lambda: rename(line_edit.text()))
        menu.addAction(action_rename)
        menu.exec(btn.mapToGlobal(pos))

    def show_context_menu_dir(self, pos, btn,name):
        menu = QMenu()
        menu.addAction("Open", lambda val=name: self.folder_function(val))
        menu.addSeparator()
        menu.addAction("Rename", lambda val=name: self.rename_menu(pos,btn,val))
        menu.addAction("Delete", lambda val=name: self.delete_dir(val))
        menu.exec(btn.mapToGlobal(pos))

    def show_context_menu_file(self, pos, btn,name):
        menu = QMenu()
        menu.addAction("Show content", lambda val=name: self.content.toggle_content_menu(val))
        menu.addSeparator()
        menu.addAction("Rename", lambda val=name: self.rename_menu(pos,btn,val))
        menu.addAction("Delete", lambda val=name: self.delete_file(val))
        menu.exec(btn.mapToGlobal(pos))



