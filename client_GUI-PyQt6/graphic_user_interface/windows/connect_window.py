from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout, QFileDialog
)
import  threading
import  time
from PyQt6.QtGui import QCursor
from PyQt6.QtCore import Qt

class ConnectWindow(QWidget):
    def __init__(self,parent_window=None):
        super().__init__()
        self.parent_window=parent_window
        self.setAutoFillBackground(True)
        self.layout = QGridLayout()
        self.layout.setContentsMargins(10,20,10,0)

        self.ip_label = QLabel("Server IP:PORT")
        self.layout.addWidget(self.ip_label, 0, 0)

        self.ip_edit = QLineEdit()
        self.ip_edit.setMaxLength(21)
        self.layout.addWidget(self.ip_edit, 0, 1)

        self.btn_select = QPushButton("Choose cert")
        self.btn_select.setObjectName("normal")
        self.btn_select.setCursor(QCursor(Qt.CursorShape.PointingHandCursor))
        self.layout.addWidget(self.btn_select, 1, 0)
        self.btn_select.clicked.connect(self.select_file)

        self.cert_path_label = QLabel("No certificate chosen")
        self.layout.addWidget(self.cert_path_label, 1, 1)

        self.connect_btn= QPushButton("Connect")
        self.connect_btn.setCursor(QCursor(Qt.CursorShape.PointingHandCursor))
        self.layout.addWidget(self.connect_btn, 2, 1)
        self.connect_btn.clicked.connect(self.try_connect)

        self.setLayout(self.layout)
        with open("graphic_user_interface/styles/connect_window.css", "r") as file:
            self.setStyleSheet(file.read())


    def try_connect(self):
        self.ip_edit.setStyleSheet(" border: 2px solid")
        self.btn_select.setStyleSheet(" border: 2px solid;")
        self.cert_path_label.setStyleSheet(" border: none")
        stop_animation=[None]
        def check_data():
            stop_animation[0]=self.parent_window.client.create_connection()==True

        def start_animation():
            from PyQt6.QtWidgets import QApplication
            count = 0
            while stop_animation[0] is None:
                text = "Connecting " + '.' * count
                self.connect_btn.setText(text)
                QApplication.processEvents()
                time.sleep(1)
                count = (count + 1) % 4

        if len(self.ip_edit.text())==0 or self.ip_edit.text().__contains__(":")==False:
            self.ip_edit.setStyleSheet(" border: 2px solid rgb(50, 128, 142);")
            return
        if self.cert_path_label.text()=="No certificate chosen":
            self.btn_select.setStyleSheet(" border: 2px solid rgb(50, 128, 142);")
            return
        ip=self.ip_edit.text().split(":")
        path=self.cert_path_label.text()
        print(ip[0],ip[1],path)
        self.parent_window.client.set_ip_port_cert(ip[0],ip[1],path)
        threading.Thread(target=check_data, daemon=True).start()
        start_animation()
        if stop_animation[0]:
            self.parent_window.show_login_window()
        else:
            self.ip_edit.setStyleSheet(" border: 2px solid rgb(109, 0, 0);")
            self.cert_path_label.setStyleSheet(" border: 2px solid rgb(109, 0, 0);")
            self.connect_btn.setText("Connect")

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


