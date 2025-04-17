var __index = {"config":{"lang":["en"],"separator":"[\\s\\-]+","pipeline":["stopWordFilter"]},"docs":[{"location":"index.html","title":"Welcome to OSHome!","text":"<p>OSHome is a single executable that allows you to integrate any device running an OS into your smart home.  It is designed to be lightweight and easy to use - similar to ESPHome.</p> <ul> <li>Execute a command on a device based on a trigger in home assistant. </li> <li>Monitor the status of a device with a custom command. </li> <li>Integrate all of your one off python scripts<sup>TM</sup> without thinking about connectivity or setting up yet another service.</li> </ul> <p>See the getting started guide for installation instructions.</p> <p>Explore the examples to see how to use OSHome.</p> <pre><code>pi@raspberrypi:~/ $ oshome\nOSHome - 0.3.9\n\nOSHome is a system which allows you to integrate any device running an OS into your smart home.\nhttps://github.com/DanielHabenicht/OSHome\n\nUsage: oshome [OPTIONS] &lt;COMMAND&gt;\n\nCommands:\n  install    Install OSHome\n  uninstall  Uninstall OSHome\n  run        Run OSHome manually.\n  help       Print this message or the help of the given subcommand(s)\n\nOptions:\n  -c, --configuration &lt;configuration_file&gt;\n          Optional configuration file. If not provided, the default configuration will be used. [default: config.yaml]\n  -h, --help\n          Print help\n  -V, --version\n          Print version\n</code></pre>"},{"location":"index.html#roadmap","title":"Roadmap","text":"<ul> <li>Monitor connected bluetooth devices and maybe even proxy them to home assistant.</li> <li> Auto installation</li> <li> Windows (https://github.com/mullvad/windows-service-rs) https://medium.com/@aleksej.gudkov/rust-windows-service-example-building-a-windows-service-in-rust-907be67d2287</li> <li> Linux Service<ul> <li>debian (https://github.com/kornelski/cargo-deb/blob/fc34c45fafc3904cadf652473ff7e9e0344c605c/systemd.md)</li> </ul> </li> <li> MacOS?</li> <li> Templates and Services</li> <li> Additional Components:</li> <li> HTTP and Web Enpoint</li> <li> BLE (https://github.com/deviceplug/btleplug)</li> <li> <p> Bluetooth Proxy (https://esphome.io/components/bluetooth_proxy.html)     https://docs.rs/bluer/latest/bluer/</p> </li> <li> <p> Custom compilation for modular builds and custom extensions.</p> </li> <li> Homeassistant Native API</li> <li> CLI for automatic generation of executables</li> <li> Builder Component similar to ESP Home</li> <li> Self update (https://github.com/jaemk/self_update)</li> <li> GPIO https://github.com/golemparts/rppal</li> </ul> <p>... Control USB Devices?</p> <p>Rust clippy:  https://github.com/rust-lang/rust-clippy</p> <p>Add Badges?  https://github.com/squidfunk/mkdocs-material/discussions/7137</p>"},{"location":"examples/index.html","title":"Overiew of all Examples","text":"<ul> <ul> <li>Check bluetooth connection </li> <li>Motion detection </li> <li>System ressources </li> </ul> </ul>"},{"location":"examples/check_bluetooth_connection/index.html","title":"Check if a Bluetooth device is connected","text":"Linux <pre><code>shell: \n\nsensor:\n  - platform: shell\n    name: \"RAM Usage\"\n    id: ram_usage\n    icon: mdi:memory\n    # device_class: \"data_size\"\n    state_class: \"measurement\"\n    unit_of_measurement: \"%\"\n    update_interval: 30s # 0 only executes it once and assumes a long running processes.\n    command: |-\n      if hcitool con | grep -q \"00:12:6F:F1:FF:61\"; then\n          echo true\n      else\n          echo false\n      fi\n</code></pre>"},{"location":"examples/motion_detection/index.html","title":"Motion Detection","text":""},{"location":"examples/system_ressources/index.html","title":"Monitor system resources","text":"WindowsLinux <pre><code>shell: \n  type: powershell\n\nsensor:\n  - platform: shell\n    name: \"RAM Usage\"\n    id: ram_usage\n    icon: mdi:memory\n    state_class: \"measurement\"\n    unit_of_measurement: \"%\"\n    update_interval: 30s\n    command: |-\n      Get-WmiObject Win32_OperatingSystem -Property * | % {([math]::Round(($_.FreePhysicalMemory)/$_.totalvisiblememorysize,2))}\n</code></pre> <pre><code>shell: \n\nsensor:\n  - platform: shell\n    name: \"RAM Usage\"\n    id: ram_usage\n    icon: mdi:memory\n    state_class: \"measurement\"\n    unit_of_measurement: \"%\"\n    update_interval: 30s\n    command: |-\n      free | grep Mem | awk '{print $3/$2 * 100.0}'\n</code></pre>"},{"location":"features/index.html","title":"Features","text":"<p>OSHome is built in a modular way. Currently the following modules are available:</p> <ul> <li> <p> Connectivity</p> <p>Connect with Home Assistant (via MQTT), REST API or simply use the built-in web server.</p> <p> Getting started</p> </li> <li> <p> Sensors</p> <p>Retrieve values or trigger an action.</p> <p> Get started</p> </li> <li> <p> Platforms</p> <p>Use the built-in platforms (shell) or create your own. </p> </li> <li> <p> More?</p> <p>More is coming soon.</p> </li> </ul>"},{"location":"features/components/index.html","title":"Components","text":"<p>Components are the building blocks of the system. Many Platform features use the base configuration options of these.</p> <ul> <li> <p> Sensor</p> <p>Make data available as a sensor.</p> <p> View Documentation</p> </li> <li> <p> Button</p> <p>Trigger an action on the device.</p> <p> View Documentation</p> </li> <li> <p> Binary Sensor</p> <p>Track on/off states or occupancy.</p> <p> View Documentation</p> </li> </ul>"},{"location":"features/components/binary_sensor.html","title":"Binary Sensor","text":"<p>TBD</p>"},{"location":"features/components/button.html","title":"Button","text":"Base Example<pre><code>button: \n - platform: ... #(1)!\n   id: my_button\n   name: \"Write Hello World\"\n</code></pre> <ol> <li>Here the plaform must be defined. </li> </ol>"},{"location":"features/components/sensor.html","title":"Sensor","text":"<pre><code>sensor:\n  - platform: shell\n    name: \"RAM Usage\"\n    id: ram_usage\n    icon: mdi:memory\n    # device_class: \"data_size\"\n    state_class: \"measurement\"\n    unit_of_measurement: \"%\"\n    update_interval: 30s # 0 only executes it once and assumes a long running processes.\n    command: |-\n      free | grep Mem | awk '{print $3/$2 * 100.0}'\n</code></pre>"},{"location":"features/connectivity/index.html","title":"Connectivity","text":"<ul> <li> <p> MQTT</p> <p>Connect to any MQTT broker</p> <p> See Documentation</p> </li> </ul>"},{"location":"features/connectivity/http.html","title":"HTTP","text":"<p>TBD</p>"},{"location":"features/connectivity/mdns.html","title":"MDNS","text":"<p>TBD</p>"},{"location":"features/connectivity/mqtt.html","title":"MQTT","text":"<p>The MQTT client connects to your MQTT broker and allows you to receive sensor updates or send actions to your device.</p> <p>This is a simple configuration: </p> <pre><code>mqtt:\n  broker: &lt;your_broker&gt;\n  username: &lt;your_username&gt;\n  password: &lt;your_password&gt;\n</code></pre> <p>Here is a more complex example:</p> <pre><code>mqtt:\n  broker: &lt;your_broker&gt;\n  username: &lt;your_username&gt;\n  password: &lt;your_password&gt;\n  discovery: true\n  discovery_prefix: homeassistant\n  topic_prefix: oshome\n</code></pre> <p>This will connect to your MQTT broker and send all sensor updates to the <code>oshome/sensor/&lt;sensor_name&gt;</code> topic. It will also listen for actions on the <code>oshome/action/&lt;action_name&gt;</code> topic.</p>"},{"location":"features/connectivity/mqtt.html#device-discovery","title":"Device Discovery","text":"<p>Device Discovery is enabled by default, so the device will automatically be discovered by Home Assistant. You can find the device in the MQTT integration in Home Assistant.</p>"},{"location":"features/connectivity/web_server.html","title":"Web Server","text":"<p>TBD</p>"},{"location":"features/platforms/index.html","title":"Platforms","text":"<ul> <li> <p> Shell</p> <p>Execute any command for retrieving data or on trigger.</p> <p> View Documentation</p> </li> <li> <p> More </p> <p>More to come...</p> </li> </ul>"},{"location":"features/platforms/bluetooth.html","title":"Bluetooth","text":"<p>TBD</p>"},{"location":"features/platforms/bme280.html","title":"BME280","text":"<p>No platform configuration is needed. Just include the following:</p> <pre><code>sensor:\n  - platform: bme280\n    name: \"\"\n</code></pre>","tags":["Temperature","Humidity","Pressure","I2C"]},{"location":"features/platforms/gpio.html","title":"GPIO","text":"<pre><code>gpio:\n  device: raspberryPi\n\nbinary_sensor:\n  - platform: gpio\n    name: \"Motion\"\n    icon: \"mdi:motion-sensor\"\n    device_class: presence\n    pin: 23\n</code></pre>"},{"location":"features/platforms/shell.html","title":"Shell","text":"<p>Enable the platform:</p> <pre><code>shell: \n  type: powershell\n</code></pre>"},{"location":"features/platforms/shell.html#usage","title":"Usage","text":""},{"location":"features/platforms/shell.html#sensors","title":"Sensors","text":"<pre><code>sensor:\n  - platform: shell\n    name: \"RAM Usage\"\n    update_interval: 30s\n    command: |-\n      free | grep Mem | awk '{print $3/$2 * 100.0}'\n</code></pre>"},{"location":"features/utilities/logger.html","title":"Logger","text":"<pre><code># Example configuration entry\nlogger:\n  level: INFO\n</code></pre> <p>From Rust LogLevels;</p>"},{"location":"getting_started/index.html","title":"Getting Started","text":"<p>For now<sup>1</sup> a single executable is provided. You can download them from the GitHub Releases page or use the direct links below.</p> LinuxWindowsMacOS Device Download Target Windows 11 ZIP Link x86_64-pc-windows-msvc Device Download Target MacOS TAR Link x86_64-apple-darwin <ol> <li> <p>Run the executable with the following command:</p> LinuxWindows <pre><code>sudo ./oshome install\n# The CLI will ask you where to install it. (You can just hit enter to install it in the default location)\n? Where do you want to install OSHome? (/usr/bin/oshome)\n</code></pre> <p>Press Win+X+A for the admin shell and run the following command:</p> <pre><code>./oshome.exe install\n# The CLI will ask you where to install it. (You can just hit enter to install it in the default location)\n? Where do you want to install OSHome? (C:\\Program Files\\oshome)\n</code></pre> <p>If you do this more often you can add the --install-path flag to the command to specify the path for the installation. Instead of the CLI asking for it.</p> </li> <li> <p>After the installation is complete you should be able to see your device in Home Assistant.:</p> </li> </ol> <ol> <li> <p>This will change in the future to allow for custom compilation for modular builds and custom extensions.\u00a0\u21a9</p> </li> </ol>"},{"location":"getting_started/index.html#download","title":"Download","text":"Device Download Target Default TAR Link x86_64-unknown-linux-musl Raspberry Pi 3 TAR Link armv7-unknown-linux-musleabi"},{"location":"getting_started/index.html#installation","title":"Installation","text":"<ol> <li> <p>Download and extract the archive and place the oshome executable in a directory of your choice.</p> <pre><code>curl -L -o oshome.tar.gz https://github.com/DanielHabenicht/OSHome/releases/download/v0.3.9/oshome-Linux-musleabi-armv7.tar.gz\ntar xvzf ./oshome.tar.gz\n./oshome\n</code></pre> </li> <li> <p>Create a configuration file in the same directory as the executable. The configuration file should be named <code>config.yaml</code> and should contain the following:</p> <pre><code>oshome:\n  name: new_awesome\n\nmqtt:\n  broker: 192.168.178.167\n  username: test\n  password: a887aeda-7248-4b0f-a2e7-6f4ee0026f5e\n\nshell:\n\nbutton:\n  - platform: shell\n    id: my_button\n    name: 'Write Hello World'\n    command: \"echo 'Hello World!' &gt;&gt; test.log\"\n\nsensor:\n  - platform: shell\n    name: \"RAM Usage\"\n    id: ram_usage\n    icon: mdi:memory\n    state_class: \"measurement\"\n    unit_of_measurement: \"%\"\n    update_interval: 30s # 0 only executes it once and assumes a long running processes.\n    command: |-\n      free | grep Mem | awk '{print $3/$2 * 100.0}'\n</code></pre> </li> <li> <p>Try it out:</p> <pre><code>./oshome run\nStarting OSHome - 0.3.9\nLogDirectory: /home/codespace/.local/share\nConfig file path: /workspaces/OSHome/config.yaml\nBinary Sensor 'bluetooth_connected' output: false\nSensor 'ram_usage' output: 38.3144\nButton 'my_button' pressed.\nCommand executed successfully with no output.\n# End the process with ctrl+c\n</code></pre> </li> <li> <p>You should be able to see your device in Home Assistant now.</p> </li> <li> <p>To persistently run the executable install it as a service:</p> <pre><code>sudo ./oshome install\n# The CLI will ask you where to install it. (You can just hit enter to install it in the default location)\n? Where do you want to install OSHome? (/usr/bin/oshome)\n</code></pre> <p>If you do this more often you can add the --install-path flag to the command to specify the path for the installation. Instead of the CLI asking for it.</p> </li> </ol>"},{"location":"getting_started/index.html#uninstallation","title":"Uninstallation","text":"<p>If you want to uninstall OSHome you can run the following command:</p> <pre><code>./oshome uninstall\n</code></pre>"},{"location":"getting_started/index.html#installation_1","title":"Installation","text":"<ol> <li>Download and extract the archive and place the oshome executable in a directory of your choice.</li> </ol> <pre><code>Invoke-WebRequest -OutFile oshome.zip -Uri https://github.com/DanielHabenicht/OSHome/releases/download/v0.3.9/oshome-Windows-msvc-x86_64.zip\nExpand-Archive -Force oshome.zip ./\n./oshome.exe\n</code></pre> <ol> <li>Create a configuration file in the same directory as the executable. The configuration file should be named <code>config.yaml</code> and should contain the following:</li> </ol> <pre><code>oshome:\n  name: new_awesome\n\nmqtt:\n  broker: 192.168.178.167\n  username: test\n  password: a887aeda-7248-4b0f-a2e7-6f4ee0026f5e\n\nshell:\n\nbutton:\n  - platform: shell\n    id: my_button\n    name: 'Write Hello World'\n    command: \"echo 'Hello World!' &gt;&gt; test.log\"\n\nsensor:\n  - platform: shell\n    name: 'RAM Usage'\n    id: ram_usage\n    icon: mdi:memory\n    state_class: 'measurement'\n    unit_of_measurement: '%'\n    update_interval: 30s # 0 only executes it once and assumes a long running processes.\n    command: |-\n      Get-WmiObject Win32_OperatingSystem -Property * | % {([math]::Round(($_.FreePhysicalMemory)/$_.totalvisiblememorysize,2))}\n</code></pre> <ol> <li> <p>Try it out:</p> <pre><code>./oshome.exe run\nStarting OSHome - 0.3.9\nLogDirectory: /home/codespace/.local/share\nConfig file path: /workspaces/OSHome/config.yaml\nBinary Sensor 'bluetooth_connected' output: false\nSensor 'ram_usage' output: 38.3144\nButton 'my_button' pressed.\nCommand executed successfully with no output.\n# End the process with ctrl+c\n</code></pre> </li> <li> <p>You should be able to see your device in Home Assistant now.</p> </li> <li> <p>To persistently run the executable install it as a service:</p> <p>Press Win+X+A for the admin shell.</p> <pre><code>./oshome.exe install\n# The CLI will ask you where to install it. (You can just hit enter to install it in the default location)\n? Where do you want to install OSHome? (C:\\Program Files\\oshome)\n</code></pre> <p>If you do this more often you can add the --install-path flag to the command to specify the path for the installation. Instead of the CLI asking for it.</p> </li> </ol>"},{"location":"getting_started/index.html#uninstallation_1","title":"Uninstallation","text":"<p>If you want to uninstall OSHome you can run the following command:</p> <pre><code>./oshome.exe uninstall\n</code></pre>"}]}