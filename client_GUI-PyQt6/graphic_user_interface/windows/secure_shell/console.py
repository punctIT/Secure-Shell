from PyQt6.QtWidgets import (
    QWidget, QPushButton, QLineEdit, QScrollArea,
    QVBoxLayout, QHBoxLayout, QTextEdit
)
from PyQt6.QtCore import Qt


class Console:
    def __init__(self, ssh):
        self.ssh = ssh
        self.output = None
        self.input_field = None
    def console_menu(self) -> QScrollArea:
        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)

        container = QWidget()
        scroll_area.setWidget(container)
        scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)

        layout = QVBoxLayout(container)
        self.output = QTextEdit()
        self.output.setReadOnly(True)
        layout.addWidget(self.output)


        input_layout = QHBoxLayout()

        self.input_field = QLineEdit()
        self.input_field.returnPressed.connect(self.run_command)

        run_btn = QPushButton("Run")
        run_btn.clicked.connect(self.run_command)

        input_layout.addWidget(self.input_field)
        input_layout.addWidget(run_btn)

        layout.addLayout(input_layout)

        self.output.append("SSH Console Ready")
        self.output.append("Type commands and press Enter")

        return scroll_area

    def run_command(self):
        command = self.input_field.text()
        if command:
            if command=="cls":
                self.output.clear()
                self.input_field.clear()
                return
            self.output.append(f"Server{self.ssh.current_path}> {command}")
            self.ssh.parent.client.sent(command)
            output=self.ssh.parent.client.receive().split("[-]")
            self.ssh.current_path = output[1]
            self.output.append(self.get_unformated_text(output[0]))
            self.input_field.clear()
            self.ssh.primary_menu.refresh_function()

    def get_unformated_text(self,text: str) -> str:
        new_text = ""
        # Split by "?&" and filter out empty strings
        props = [p for p in text.split("?&") if p]

        for w in props:
            # Extract parts after first character, split by "\n\n"
            word = [part for part in w[1:].split("\n\n") if part]

            # Get first character of segment
            first_char = w[0] if w else None

            if first_char == 'C':
                for e in word:
                    i = 0
                    chars = list(e)
                    while i < len(chars):
                        c = chars[i]

                        # Skip "^@" pattern
                        if i + 1 < len(chars) and c == '^' and chars[i + 1] == '@':
                            i += 2
                            continue

                        # Skip "~~" pattern
                        if i + 1 < len(chars) and c == '~' and chars[i + 1] == '~':
                            i += 2
                            continue

                        # Append character
                        new_text += c
                        i += 1

            elif first_char is not None:
                for e in word:
                    if e.startswith('^'):
                        # Skip first 2 characters
                        tail = e[2:]
                        if not new_text:
                            new_text = tail
                        else:
                            new_text = f"{new_text} {tail}"
                    else:
                        if not new_text:
                            new_text = e
                        else:
                            new_text = f"{new_text} {e}"

        return new_text