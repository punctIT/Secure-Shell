from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLabel, QLineEdit, QGridLayout,QScrollArea, QFileDialog, QMenu
)

from file_area import get_files_area
from left_menu import secundary_menu
from console import console_menu
test_text='?&L\n\n^!1\n\n^#1.exe\n\n^!12\n\n^!13\n\n20240824_150239.jpg\n\n20241220_104800.jpg\n\n^!acte faculate\n\nalegeri.txt\n\nalex_s_1.pptx\n\n^!asm\n\n^!Certificate\n\n^!cgit\n\n^!ConsoleApplication1\n\nCounter-Strike 2.url\n\n^!cursuri\n\ndesktop.ini\n\nDesktop.rar\n\ndocument.pdf\n\nFII PA S16 results.pdf\n\nFilme & seriale vizionate.xlsx\n\n^!gdt050579.github.io\n\ngdt050579.github.io.zip\n\n^!git-clone-rust-main\n\ngit-clone-rust-main.zip\n\n^!inutil\n\n^!Laborator\n\nLaborator.zip\n\nlogo ista.png\n\nmeme.png\n\n^!New folder\n\n^!New folder (2)\n\nNew Text Document.txt\n\n^!oop ses\n\npassword.txt\n\npc-garage-srl-factura_fiscala_13017852.pdf\n\nProiecte python A.pdf\n\nProiecte python B.pdf\n\nProiecte python C.pdf\n\n^!py_rc\n\n^!RC\n\n^!Redis Gui Client\n\nRockstar Games Launcher.lnk\n\n^!rust site\n\n^!s\n\nScreenshot 2025-07-25 210733.png\n\n^!Secure Shell Rust\n\nstats.txt\n\n^!Tema 1\n\nTeme Laborator.zip\n\n^!unity\n\nuntitled.blend\n\nwetransfer_bootcamp-level-design-pdf_2023-07-13_1140.zip\n\nWhatsApp Image 2023-01-12 at 21.17.26.jpg\n\n~$dele bac 2020.docx\n\n~$me_proiect_arbori.docx\n\n[-]:[-]\r\n\r\n'



class SecureShellWindow(QWidget):
    def __init__(self, parent_window=None):
        super().__init__()
        self.parent=parent_window
        self.layout = QGridLayout()
        self.primary_layout=QGridLayout()

        self.console_btn=QPushButton("Console")
        self.console_status=True
        self.console_widget = None
        self.console_btn.clicked.connect(self.toggle_console)
        self.layout.addWidget(self.console_btn,0,0)

        self.primary_layout.addLayout(secundary_menu(self),1,0)

        self.setLayout(self.layout)
        self.primary_layout.addWidget(get_files_area(self,test_text),1,1)


        self.layout.addLayout(self.primary_layout,1,0)

        with open("ssh_window.css") as file:
            self.setStyleSheet(file.read())

    def toggle_console(self):
        if self.console_status:
            if not self.console_widget:
                wrapper = QWidget()
                wrapper.setLayout(console_menu(self))
                self.console_widget = wrapper
                self.layout.addWidget(self.console_widget, 2, 0)
        else:
            if self.console_widget:
                self.console_widget.setParent(None)
                self.console_widget.deleteLater()
                self.console_widget = None
        self.console_status = not self.console_status






if __name__=="__main__":
    import sys
    from PyQt6.QtWidgets import QApplication
    app = QApplication(sys.argv)
    window = SecureShellWindow()
    window.show()
    sys.exit(app.exec())