<script lang='ts'>
    export let wasm;
	let words = JSON.parse(wasm.get_best());
	let definitions = {};

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
	
	async function scrape(item) {
		let search = item.word
		let doc = parser.parseFromString(await (await fetch(BASE_URL + search + '%23words')).text(), 'text/html');
		
		// doc.querySelectorAll('div.exact_block > div') // no concepts
		doc.querySelectorAll('#primary > div > div').forEach(entry => {
			let sentence = [];

			let furigana = entry.querySelector('.concept_light-representation > .furigana').childNodes;
			let kanji = entry.querySelector('.concept_light-representation > .text').childNodes;

			for (let index=0; index < kanji.length; index++) {
				if (furigana[index + 1] != null && furigana[index + 1].textContent.trim() != '') {
					sentence.push({
						'kanji': kanji[index].textContent.trim(),
						'furigana': furigana[index + 1].textContent.trim()
					});
				} else if (kanji[index] != null && kanji[index].textContent.trim() != '') {
					sentence.push({
						'kanji': kanji[index].textContent.trim(),
						'furigana': ''
					});
				}
			}

			let entries = map(entry.querySelectorAll('.meanings-wrapper'), meaningEntry => {
				let types = getIterableText(meaningEntry.querySelectorAll('.meaning-tags'));

				return map(meaningEntry.querySelectorAll('.meaning-wrapper'), (meaning, index) => {
					let type = types[index];
					let definition = getIterableText(meaning.querySelectorAll('.meaning-meaning'));
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
						'definition': definition,
						'types': type,
						'senses': senses,
						'english_sentence': english_sentence,
						'japanese_sentence': japanese_sentence
					}
				});
			});

			if (search in definitions) {
				definitions[search].push({
					'sentence': sentence,
					'entries': entries,
					'frequency': item.frequency
				});
			} else {
				definitions[search] = [{
					'sentence': sentence,
					'entries': entries,
					'frequency': item.frequency
				}];
			}
		})
	}
	
	words.forEach(element => {
		scrape(element).catch(_ => console.log(element.word));
	});
</script>

<div class='flex flex-col gap-5 self-center text-center'>
	{#each Object.entries(definitions) as [search, results]}
		<div class='flex flex-col cursor-pointer rounded-lg bg-gradient-to-r from-red-600 via-pink-300 to-red-600'>
			<div class='flex flex-row w-full items-center'>
				<div class='flex flex-col w-full items-center'>
					<div class='flex flex-row items-end p-5'>
						{#each results[0]['sentence'] as sentence}
							<div class='text-5xl'>
								<p>{sentence['furigana']}</p>
								<p>{sentence['kanji']}</p>
							</div>
						{/each}
					</div>
				</div>
				<div>
					<p class='bg-yellow-600'>{search}</p>
					<p class='bg-red-600'>{results[0]['frequency']}</p>
				</div>
			</div>
			
			<div class='flex flex-col w-full items-center rounded-b-lg bg-red-400'>
				{#each results[0]['entries'] as entry}
					<div class='w-9/12'>
						{#each entry as meaning}
							<div class='flex flex-row p-3 items-center text-xs'>
								<p class='bg-blue-300'>{meaning['types']}</p>
								<div class='px-3 text-base'>
									{#each meaning['definition'] as definition}
										<p>{definition}</p>
									{/each}
								</div>
								<div class='bg-yellow-300'>
									{#each meaning['senses'] as sense}
										<p>{sense}</p>
									{/each}
								</div>
							</div>
						{/each}
					</div>
				{/each}
			</div>
		</div>
	{/each}
</div>

<style global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>