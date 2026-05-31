# Ахитектура проекта

- cli
- corelib
    1. audio
    2. player
    3. track
    4. queue
    5. playlist

<hr>

## cli

CLI интерфейс для плеера

<hr>

## corelib

### track

Реализована структура Track и Metadata.

*const SUPPORTED_EXTENSIONS* - массив доступных расширений файлов.

*read_metadata(&Path) -> Metadata* - (supporting function) получение метаданных из трека

*Track::from_path(PathBuf) -> Result<Self>* - читает трек по пути и возвращает его в виде Result<Track>.

### playlist

Структура Playlist. 

*Playlist::save(&self, &Path) -> anyhow::Result<()>* - сохранение плейлиста по пути

*Playlist::load(&Path) -> anyhow::Result<Self>* - загрузить плейлист в очередь

*Playlist::add_track(&mut self, Track)* - добавить трек в плейлист (по пути)

*Playlist::remove_track(&mut self, usize) -> Option<Track>* - удалить трек из плейлиста (возвращает удалённый Track)

*Playlist::clear(&mut self)* - очистить плейлист

*Playlist::delete_playlist(&Path) -> anyhow::Result<()>* - удалить плейлист (файл)

*Playlist::add_tracks_from_queue(&mut self, &queue::Queue)* - добавить в плейлист треки из очереди

*Playlist::add_tracks_from_folder(&mut self, &Path) -> anyhow::Result<()>* - добавить в плейлист треки из папки

### queue

Реализована структура Queue

*Queue::load_from_folder(&Path) -> anyhow::Result<Self>* - загрузить треки в очереди из папки

*Queue::next_track(&mut self)* - проиграть следующий трек

*Queue::previous_track(&mut self)* - проиграть предыдущий трек

*Queue::load_playlist(&Path) -> anyhow::Result<Self>* загрузить плейлист в очередь

*Queue::add_from_playlist(&mut self, &Path) -> Result<()>* - добавить в очередь треки из плейлиста

*Queue::shuffle(&mut self)* - перемешать очередь

*Queue::unshuffle(&mut self)* - отменить перемешивание

*Queue::set_current(&mut self, usize)* - установить текущий трек

*Queue::load_track_from_path(&mut self, PathBuf) -> anyhow::Result<()>* - загрузить в очередь трек из папки

*Queue::clear(&mut self)* - очистить очередь

### audio

Реализована структура RodioBackend

*RodioBackend::new() -> Result<Self>*

*RodioBackend::seek(&self, Duration) -> Result<()>*

*RodioBackend::is_finished(&self) -> bool*

*RodioBackend::reset_sink(&mut self)*

*RodioBackend::play_file(&mut self, &Path) -> Result<Option<Duration>>*

*RodioBackend::pause(&self)*

*RodioBackend::play(&self)*

*RodioBackend::stop(&self)*

*RodioBackend::set_volume(&self, f32)*

*RodioBackend::is_paused(&self)*

*RodioBackend::is_empty(&self)*

### player

Реализованы структура Player и множества состояний PlaybackState и RepeatMode

*Player::new(Queue) -> Result<Self>*

*Player::seek(&mut self, u64) -> Result<()>*

*Player::save_playlist(&self, &Path) -> Result<()>*

*Player::load_playlist(&mut self, &Path) -> Result<()>*

*Player::update(&mut self) -> Result<()>*

*Player::shuffle(&mut self)*

*Player::unshuffle(&mut self)*

*Player::set_repeat_mode(&mut self, RepeatMode)*

*Player::repeat_mode(&self) -> RepeatMode*

*Player::play_current(&mut self) -> Result<()>*

*Player::play_track(&mut self, usize) -> Result<()>*

*Player::current_tracn_name(&self) -> Option<String>*

*Player::get_track_info(&self) -> Option<String>*

*Player::next_track(&mut self) -> Result<()>*

*Player::previous_track(&mut self) -> Result<()>*

*Player::pause(&mut self)*

*Player::resume(&mut self)*

*Player::stop(&mut self)*

*Player::set_volume(&self, f32)*

*Player::state(&self) -> PlaybackState*

*Player::queue(&self) -> &Queue*