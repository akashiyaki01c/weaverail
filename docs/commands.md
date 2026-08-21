# コマンド定義

太字は実装済

## 駅系

- **AddStation** (駅を追加する)
	- ID重複時エラー
- **DeleteStation** (駅を削除する)
	- 参照先不明時エラー
	- Lineから被参照時エラー
- **RenameStation** (駅名を変更する)
	- 参照先不明時エラー
- **AddTrack** (番線を追加する)
	- 指定駅不明時エラー
	- ID重複時エラー
- DeleteTrack (番線を削除する)
	- 指定駅不明時エラー
	- 参照先不明時エラー
	- TemplateTrainから被参照時エラー
- RenameTrack (番線名を変更する)
	- 指定駅不明時エラー
	- 参照先不明時エラー

## 路線系

- **AddLine** (路線を追加する)
	- ID重複時エラー
- **DeleteLine** (路線を削除する)
	- 参照先不明時エラー
	- 内包SegmentがTemplateTrainから被参照時エラー
- **RenameLine** (路線名を変更する)
- **PushBackSegment** (駅間を末尾に追加する)
	- ID重複時エラー
- **PushFrontSegment** (駅間を先頭に追加する)
	- 駅が路線に既に含まれている場合エラー
- PopBackSegment (末尾の駅間を削除する)
	- 駅間がTemplateTrainから参照されている場合エラー
- PopFrontSegment (先頭の駅間を削除する)
	- 駅間がTemplateTrainから参照されている場合エラー
- ReplaceSegment (駅間を置換する、A→BをA→C→B、その反対も然り)

## 列車種別系

- **AddTrainType** (列車種別を追加する)
	- ID重複時エラー
- **DeleteTrainType** (列車種別を削除する)
	- 参照先不明時エラー
	- TemplateTrainから被参照時エラー
- RenameTrainType (列車種別名を変更する)

## テンプレート列車系

- **AddTemplateTrain** (テンプレート列車を追加する)
- DeleteTemplateTrain (テンプレート列車を削除する)
- RenameTemplateTrain (テンプレート列車名を変更する)
- **PushBackTemplateTrainSegment** (テンプレート列車の末尾に走行区間を追加する)
- PushFrontTemplateTrainSegment (テンプレート列車の先頭に走行区間を追加する)
- PopBackTemplateTrainSegment (テンプレート列車の末尾の走行区間を削除する)
- PopFrontTemplateTrainSegment (テンプレート列車の先頭の走行区間を削除する)
- ReplaceTemplateTrainSegment (テンプレート列車の走行区間を置換する)

## 時刻表系

- **AddTimetable** (時刻表を追加する)
- DeleteTimetable (時刻表を削除する)
- RenameTimetable (時刻表名を変更する)
- ChangeTrainOrder (列車順序を変更する)

## 列車系

- **AddTrain** (列車を追加する)
- DeleteTrain (列車を削除する)
- RenameTrain (列車名を変更する)
- ChangeDepartureTime (始発時刻を変更する)
- (列車の運行区間を短縮する)
- (列車の運行区間を延長する)
- (テンプレート列車を置換する)
