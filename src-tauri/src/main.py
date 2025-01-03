from portablemc.fabric import FabricVersion
from portablemc.auth import OfflineAuthSession
from portablemc.standard import Version, Path, Context, Watcher, VersionLoadingEvent, VersionLoadedEvent, FeaturesEvent, JvmLoadingEvent, JvmLoadedEvent, JarFoundEvent, AssetsResolveEvent, LibrariesResolvingEvent, LibrariesResolvedEvent, LoggerFoundEvent
import os
import uuid

class MyWatcher(Watcher):
    def handle(self, event) -> None:
        # Проверяем тип события и выводим соответствующее сообщение
        if isinstance(event, VersionLoadingEvent):
            print("Загрузка версии...")
        elif isinstance(event, VersionLoadedEvent):
            print("Версия загружена.")
        elif isinstance(event, FeaturesEvent):
            print("Загрузка функций...")
        elif isinstance(event, JvmLoadingEvent):
            print("Загрузка JVM...")
        elif isinstance(event, JvmLoadedEvent):
            print("JVM загружена.")
        elif isinstance(event, JarFoundEvent):
            print("JAR файл найден.")
        elif isinstance(event, AssetsResolveEvent):
            print("Разрешение ассетов...")
        elif isinstance(event, LibrariesResolvingEvent):
            print("Разрешение библиотек...")
        elif isinstance(event, LibrariesResolvedEvent):
            print("Библиотеки разрешены.")
        elif isinstance(event, LoggerFoundEvent):
            print("Логгер найден.")
        else:
            print(f"Неизвестное событие: {event}")

main_directory = Path("../")
work_directory = Path("../")
context = Context(main_directory, work_directory)

fabric_version = FabricVersion.with_fabric("1.21.4", "0.16.9", context=context)

auth_session = OfflineAuthSession("YourNickname", str(uuid.uuid4()))

fabric_version.auth_session = auth_session

env = fabric_version.install(watcher=MyWatcher())
env.run()
