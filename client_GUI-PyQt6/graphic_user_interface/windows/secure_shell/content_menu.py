from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu,QSizePolicy
)
from PyQt6.QtGui import QIcon,QCursor
from PyQt6.QtCore import Qt,QSize

class Content:
    def __init__(self,ssh):
        self.ssh=ssh
        self.content_status=False

    def get_content_menu(self, text):

        content_widget = QWidget()
        layout = QGridLayout(content_widget)

        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)
        container = QWidget()
        scroll_area.setWidget(container)
        scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)

        scroll_grid = QGridLayout(container)
        self.ssh.parent.client.sent(f"cat \"{text}\"")
        content = self.ssh.parent.client.receive().split("[-]")[0][4:]
        if len(content.strip())==0:
            content="No data could be read from the file. It may be empty or access is restricted."
        content_lbn = QLabel(content)
        content_lbn.setWordWrap(True)
        content_lbn.setAlignment(Qt.AlignmentFlag.AlignLeft | Qt.AlignmentFlag.AlignTop)
        scroll_grid.addWidget(content_lbn, 0, 0)

        exit_btn = QPushButton()
        exit_btn.clicked.connect(self.close_content)
        exit_btn.setSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Expanding)
        exit_btn.setCursor(QCursor(Qt.CursorShape.PointingHandCursor))
        exit_btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/content_close.png"))
        exit_btn.setFixedWidth(25)

        layout.setColumnStretch(0, 1)
        layout.setColumnStretch(1, 99)
        layout.addWidget(exit_btn, 0, 0)
        layout.addWidget(scroll_area, 0, 1)

        return content_widget
    def close_content(self):
        item = self.ssh.primary_layout.itemAtPosition(0, 2)
        if item is not None:
            widget = item.widget()
            if widget is not None:
                widget.setParent(None)
                widget.deleteLater()
                self.ssh.primary_layout.setColumnStretch(0, 10)
                self.ssh.primary_layout.setColumnStretch(1, 90)
                self.ssh.primary_layout.setColumnStretch(2, 0)
                container = self.ssh.primary_layout.parentWidget()
                if container:
                    container.adjustSize()
        self.content_status = not self.content_status

    def toggle_content_menu(self,name):
        if not self.content_status:
            item = self.ssh.primary_layout.itemAtPosition(0, 2)
            if item is None:
                self.ssh.primary_layout.addWidget(self.get_content_menu(name), 0, 2)
                self.ssh.primary_layout.setColumnStretch(0, 10)
                self.ssh.primary_layout.setColumnStretch(1, 50)
                self.ssh.primary_layout.setColumnStretch(2, 40)

        else:
            item = self.ssh.primary_layout.itemAtPosition(0, 2)
            if item is not None:
                widget = item.widget()
                if widget is not None:
                    widget.setParent(None)
                    widget.deleteLater()
                    self.ssh.primary_layout.addWidget(self.get_content_menu(name), 0, 2)
        self.content_status=True

