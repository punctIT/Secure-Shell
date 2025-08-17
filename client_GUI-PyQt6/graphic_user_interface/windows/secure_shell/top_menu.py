from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu,QApplication, QMessageBox,
QVBoxLayout
)


from PyQt6.QtGui import QIcon
from PyQt6.QtCore import Qt,QSize


class PrimaryMenu:
    def __init__(self,ssh):
        self.ssh=ssh
        self.path_label = QLabel("")
    def primary_menu(self) -> QGridLayout:
        layout = QGridLayout()
        layout.setColumnStretch(0, 3)
        layout.setColumnStretch(1, 3)
        layout.setColumnStretch(2,3)
        layout.setColumnStretch(3, 81)
        layout.setColumnStretch(4, 4)
        user_btn = QPushButton()
        user_btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/user.png"))
        user_btn.setIconSize(QSize(45, 45))
        user_btn.clicked.connect(lambda checked, b=user_btn: self.show_context_menu(b))
        layout.addWidget(user_btn, 0, 4)

        back_btn = QPushButton()
        back_btn.clicked.connect(self.back_function)
        back_btn.setObjectName("topmenubtn")
        back_btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/backward.png"))
        back_btn.setIconSize(QSize(20, 20))
        layout.addWidget(back_btn, 0, 0)

        forward_btn = QPushButton()
        forward_btn.clicked.connect(self.forward_funtion)
        forward_btn.setObjectName("topmenubtn")
        forward_btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/forward.png"))
        forward_btn.setIconSize(QSize(20, 20))
        layout.addWidget(forward_btn, 0, 1)

        refresh_btn= QPushButton()
        refresh_btn.clicked.connect(self.refresh_function)
        refresh_btn.setObjectName("topmenubtn")
        refresh_btn.setIcon(QIcon("graphic_user_interface/Assets/Icons/refresh.png"))
        refresh_btn.setIconSize(QSize(20, 20))
        layout.addWidget(refresh_btn, 0, 2)

        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)

        container = QWidget()
        scroll_area.setWidget(container)
        scroll_area.setObjectName("CurrentPath")
        scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)
        scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        scroll_grid = QGridLayout(container)
        self.path_label = QLabel(f"HOME\\{self.ssh.current_path}")
        scroll_grid.addWidget(self.path_label, 0, 0)
        scroll_area.setFixedHeight(35)
        layout.addWidget(scroll_area, 0, 3)
        return layout

    def show_context_menu(self, button):
        menu = QMenu()
        menu.addAction("Active Users",self.show_active_users )
        menu.addAction("Log Out", self.log_off)
        menu.addAction("Exit Server",self.exit_server)
        menu.addSeparator()
        menu.exec(button.mapToGlobal(button.rect().bottomLeft()))

    def show_active_users(self):
        from datetime import datetime
        current_time = datetime.utcnow().strftime("%Y-%m-%d %H:%M:%S")

        msg_box = QMessageBox()
        msg_box.setWindowTitle("Active Users")

        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)
        scroll_area.setMinimumHeight(200)
        scroll_area.setMinimumWidth(250)
        container = QWidget()
        container_layout = QVBoxLayout(container)
        self.ssh.parent.client.sent("who")
        active_users = self.ssh.parent.client.receive().split("[-]")[0][4:].split("\n\n")
        for user in active_users:
            user_label = QLabel(user)
            user_label.setStyleSheet("padding: 5px; border-bottom: 1px solid #ccc;")
            container_layout.addWidget(user_label)

        scroll_area.setWidget(container)
        layout = msg_box.layout()
        layout.addWidget(scroll_area, 1, 0, 1, layout.columnCount())

        msg_box.setStandardButtons(QMessageBox.StandardButton.Ok)
        msg_box.exec()
    def log_off(self):
        self.ssh.parent.client.log_off()
        self.ssh.parent.show_login_window()
    def exit_server(self):
        self.ssh.parent.client.terminate_connection()
        self.ssh.parent.show_connect_window()
    def update_path_label(self):
        self.path_label.setText(f"HOME\\{self.ssh.current_path}")
    def back_function(self):
        self.ssh.file_area.folder_function("..")
    def refresh_function(self):
        self.ssh.update_path()
        self.ssh.file_area.update_file_area()

    def forward_funtion(self):
        print("forward")






