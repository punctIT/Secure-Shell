import socket
import ssl


class TlsClient:
    def __init__(self):
        self.port=None
        self.ip=None
        self.hostname="localhost"
        self.server_cert_path=None

        self.ssock=None
    def set_ip_port_cert(self,ip,port,cert_path):
        self.ip=ip
        self.port=port
        self.server_cert_path=cert_path
    def create_connection(self)->bool:
        context=ssl.create_default_context(ssl.Purpose.SERVER_AUTH)
        context.load_verify_locations(self.server_cert_path)

        try:
            sock=socket.create_connection((self.ip,self.port))
            self.ssock=context.wrap_socket(sock, server_hostname=self.hostname)

        except Exception as e:
            print(f"[connection error] {e}")
            return False
        return True
    def log_off(self):
        self.ssock.close()
        self.create_connection()
    def terminate_connection(self):
        self.ssock.close()
        self.ssock = None
        self.port = None
        self.ip = None
        self.hostname = "localhost"
        self.server_cert_path = None

    def sent(self,message):
        self.ssock.sendall(message.encode("utf-8"))
    def receive4096(self)->str:
        answer = self.ssock.recv(4096).decode("utf-8")
        return answer
    def receive(self)->str:
        buffer = b""
        terminator = b"\r\n\r\n"
        while True:
            chunk = self.ssock.recv(4096)
            if not chunk:
                break
            buffer += chunk
            if terminator in buffer:
                break

        answer = buffer.decode("utf-8", errors="ignore")
        return answer











