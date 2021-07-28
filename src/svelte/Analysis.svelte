<script lang='ts'>
    export let wasm;
	let words = JSON.parse(wasm.get_best());

	const CORS_PROXY = 'https://cors.bridged.cc/';
	const JISHO_SEARCH = 'https://jisho.org/search/';
	const BASE_URL = CORS_PROXY + JISHO_SEARCH;
	let parser = new DOMParser();

	function map(iterable, func) {
		return Array.from(iterable).map(func);
	}

	function getIterableText(iterable) {
		return map(iterable, entry => entry.textContent.trim());
	}
	
	async function scrape(search) {
		let doc = parser.parseFromString(await (await fetch(BASE_URL + search + '%23words')).text(), 'text/html');
		
		// doc.querySelectorAll('div.exact_block > div'); // no concepts
		return map(doc.querySelectorAll('#primary > div > div'), entry => {
			let furigana = entry.querySelector('span.furigana > span, rt').textContent.trim();
			let kanji = entry.querySelector('.concept_light-representation > span.text').textContent.trim();

			let entries = map(entry.querySelectorAll('.meanings-wrapper'), meaningEntry => {
				let types = getIterableText(meaningEntry.querySelectorAll('.meaning-tags'));

				let meanings = map(meaningEntry.querySelectorAll('.meaning-wrapper'), meaning => {
					let definitions = getIterableText(meaning.querySelectorAll('.meaning-meaning'));
					let senses = getIterableText(meaning.querySelectorAll('.sense-tag'));
					
					let sentence = meaning.querySelector('.sentence > ul');

					let english_sentence, japanese_sentence;
					let furigana_sentence, unlinked_sentence;

					if (sentence != null) {
						english_sentence = sentence.querySelector('.english').textContent;

						japanese_sentence = map(sentence.querySelectorAll('.clearfix'), part => {
							let furigana_sentence_ = part.querySelector('.furigana');
							let unlinked_sentence_ = part.querySelector('.unlinked');

							furigana_sentence = furigana_sentence_ != null ? furigana_sentence_.textContent.trim() : '';
							unlinked_sentence = unlinked_sentence_ != null ? unlinked_sentence_.textContent.trim() : '';

							return {
								'furigana_sentence': furigana_sentence,
								'unlinked_sentence': unlinked_sentence
							}
						})
					} else {
						english_sentence = '';
						japanese_sentence = '';

						furigana_sentence = '';
						unlinked_sentence = '';
					}

					return {
						'definitions': definitions,
						'senses': senses,
						'english_sentence': english_sentence,
						'japanese_sentence': japanese_sentence
					}
				});

				return {
					'types': types,
					'meanings': meanings
				}
			});

			return {
				'furigana': furigana,
				'kanji': kanji,
				'entries': entries
			}
		})
	}
	
	words.forEach(element => {
		scrape(element.word).catch(_ => console.log(element.word));
	});
</script>

<div class='flex flex-col self-center text-center'>
	{#each words as word}
		<div class='flex flex-row justify-between cursor-pointer rounded-lg m-3 bg-gradient-to-r from-red-600 via-pink-300 to-red-600 text-5xl'>
			<p class='p-3'>{word['word']}</p>
			<p class='rounded-r-lg w-36 p-3 bg-red-400'>{word['frequency']}</p>
		</div>
	{/each}
</div>

<style global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>