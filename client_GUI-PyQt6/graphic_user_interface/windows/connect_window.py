from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout, QFileDialog
)
from graphic_user_interface.windows.login_window import LoginWindow
class ConnectWindow(QWidget):
    def __init__(self,parent_window=None):
        super().__init__()
        self.parent_window=parent_window
        self.layout = QGridLayout()
        self.ip_label = QLabel("Server IP:PORT: ")
        self.layout.addWidget(self.ip_label, 0, 0)

        self.ip_edit = QLineEdit()
        self.layout.addWidget(self.ip_edit, 0, 1)

        btn_select = QPushButton("Choose cert")
        self.layout.addWidget(btn_select, 1, 0)
        btn_select.clicked.connect(self.select_file)

        self.cert_path_label = QLabel("No certificate chosen")
        self.layout.addWidget(self.cert_path_label, 1, 1)

        connect_btn= QPushButton("Connect")
        self.layout.addWidget(connect_btn, 2, 1)
        connect_btn.clicked.connect(self.try_connect)

        self.setLayout(self.layout)
    def try_connect(self):
        if len(self.ip_edit.text())==0:
            return
        ip=self.ip_edit.text().split(":")
        path=self.cert_path_label.text()
        print(ip[0])
        print(ip[1])
        print(path)
        self.parent_window.client.set_ip_port_cert(ip[0],ip[1],path)

        if self.parent_window.client.create_connection()==True:
            self.parent_window.show_login_window()
    def select_file(self):
        file_path, _ = QFileDialog.getOpenFileName(
            self,
            "Choose a File",
            "",
            "Certificate Files (*.crt *.cer *.pem *.der);;All Files (*)"
        )
        if file_path:
            self.cert_path_label.setText(file_path)
        else:
            self.cert_path_label.setText("No certificate chosen")